use sqlx::PgPool;

use crate::dto::estoque::*;
use crate::errors::AppError;
use crate::models::produto::Produto;
use crate::services::auditoria_service::AuditoriaService;

pub struct EstoqueService;

impl EstoqueService {
    /// Lista produtos em estoque filtrados por role
    /// Admin: todos, Produtor: proprios (via granja), Financeiro: bloqueado
    pub async fn list(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
    ) -> Result<Vec<ProdutoResponseDto>, AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem acessar estoque.".into(),
            ));
        }

        let produtos = match user_role {
            "Administrador" => {
                sqlx::query_as::<_, Produto>(
                    r#"SELECT "Id", "Nome", "Tipo", "Quantidade", "UnidadeDeMedida", "GranjaId"
                       FROM "Produtos"
                       ORDER BY "Id""#,
                )
                .fetch_all(pool)
                .await?
            }
            "Produtor" => {
                sqlx::query_as::<_, Produto>(
                    r#"SELECT p."Id", p."Nome", p."Tipo", p."Quantidade",
                              p."UnidadeDeMedida", p."GranjaId"
                       FROM "Produtos" p
                       INNER JOIN "Granjas" g ON g."Id" = p."GranjaId"
                       WHERE g."UsuarioId" = $1
                       ORDER BY p."Id""#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await?
            }
            _ => Vec::new(),
        };

        let dtos = produtos
            .into_iter()
            .map(|p| ProdutoResponseDto {
                id: p.id,
                nome: p.nome,
                tipo: p.tipo,
                quantidade: p.quantidade,
                unidade_de_medida: p.unidade_de_medida,
                granja_id: p.granja_id,
            })
            .collect();

        Ok(dtos)
    }

    /// Cria novo produto em estoque
    pub async fn create(
        pool: &PgPool,
        dto: &CreateProdutoDto,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<ProdutoResponseDto, AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem criar produtos.".into(),
            ));
        }

        Self::verificar_acesso_granja(pool, dto.granja_id, user_id, user_role).await?;

        let produto = sqlx::query_as::<_, Produto>(
            r#"INSERT INTO "Produtos" ("Nome", "Tipo", "Quantidade", "UnidadeDeMedida", "GranjaId")
               VALUES ($1, $2, $3, $4, $5)
               RETURNING "Id", "Nome", "Tipo", "Quantidade", "UnidadeDeMedida", "GranjaId""#,
        )
        .bind(&dto.nome)
        .bind(&dto.tipo)
        .bind(dto.quantidade)
        .bind(&dto.unidade_de_medida)
        .bind(dto.granja_id)
        .fetch_one(pool)
        .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "CRIACAO_PRODUTO",
            &format!(
                "Produto '{}' (ID: {}) adicionado ao estoque da Granja ID: {}.",
                produto.nome, produto.id, produto.granja_id
            ),
        )
        .await?;

        Ok(ProdutoResponseDto {
            id: produto.id,
            nome: produto.nome,
            tipo: produto.tipo,
            quantidade: produto.quantidade,
            unidade_de_medida: produto.unidade_de_medida,
            granja_id: produto.granja_id,
        })
    }

    /// Atualiza produto em estoque
    pub async fn update(
        pool: &PgPool,
        id: i32,
        dto: &UpdateProdutoDto,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<ProdutoResponseDto, AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem editar produtos.".into(),
            ));
        }

        // Busca produto existente
        let existente = sqlx::query_as::<_, Produto>(
            r#"SELECT "Id", "Nome", "Tipo", "Quantidade", "UnidadeDeMedida", "GranjaId"
               FROM "Produtos"
               WHERE "Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Produto com ID {} nao encontrado", id)))?;

        // Verifica acesso a granja do produto
        Self::verificar_acesso_granja(pool, existente.granja_id, user_id, user_role).await?;

        let produto = sqlx::query_as::<_, Produto>(
            r#"UPDATE "Produtos"
               SET "Nome" = $1, "Tipo" = $2, "Quantidade" = $3,
                   "UnidadeDeMedida" = $4, "GranjaId" = $5
               WHERE "Id" = $6
               RETURNING "Id", "Nome", "Tipo", "Quantidade", "UnidadeDeMedida", "GranjaId""#,
        )
        .bind(&dto.nome)
        .bind(&dto.tipo)
        .bind(dto.quantidade)
        .bind(&dto.unidade_de_medida)
        .bind(dto.granja_id)
        .bind(id)
        .fetch_one(pool)
        .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "ATUALIZACAO_PRODUTO",
            &format!("Produto '{}' (ID: {}) atualizado.", produto.nome, id),
        )
        .await?;

        Ok(ProdutoResponseDto {
            id: produto.id,
            nome: produto.nome,
            tipo: produto.tipo,
            quantidade: produto.quantidade,
            unidade_de_medida: produto.unidade_de_medida,
            granja_id: produto.granja_id,
        })
    }

    /// Deleta produto em estoque
    pub async fn delete(
        pool: &PgPool,
        id: i32,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<(), AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem deletar produtos.".into(),
            ));
        }

        let produto = sqlx::query_as::<_, Produto>(
            r#"SELECT "Id", "Nome", "Tipo", "Quantidade", "UnidadeDeMedida", "GranjaId"
               FROM "Produtos"
               WHERE "Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Produto com ID {} nao encontrado", id)))?;

        Self::verificar_acesso_granja(pool, produto.granja_id, user_id, user_role).await?;

        sqlx::query(r#"DELETE FROM "Produtos" WHERE "Id" = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "DELECAO_PRODUTO",
            &format!("Produto '{}' (ID: {}) deletado.", produto.nome, id),
        )
        .await?;

        Ok(())
    }

    /// Verifica se o usuario tem acesso a granja especificada
    async fn verificar_acesso_granja(
        pool: &PgPool,
        granja_id: i32,
        user_id: i32,
        user_role: &str,
    ) -> Result<(), AppError> {
        match user_role {
            "Administrador" => Ok(()),
            "Produtor" => {
                let is_owner: bool = sqlx::query_scalar(
                    r#"SELECT EXISTS(
                        SELECT 1 FROM "Granjas"
                        WHERE "Id" = $1 AND "UsuarioId" = $2
                    )"#,
                )
                .bind(granja_id)
                .bind(user_id)
                .fetch_one(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                if is_owner {
                    Ok(())
                } else {
                    Err(AppError::Forbidden(
                        "Permissao negada ou granja invalida.".into(),
                    ))
                }
            }
            _ => Err(AppError::Forbidden("Perfil nao reconhecido.".into())),
        }
    }
}
