/**
 * Profile (Perfil) constants matching the seeded database values.
 *
 * These IDs correspond to rows in the `Perfis` table seeded by the backend:
 *   1 = Administrador, 2 = Produtor, 3 = Financeiro
 *
 * If backend seed data changes, update these values accordingly.
 */

export const PERFIL_IDS = {
  Administrador: 1,
  Produtor: 2,
  Financeiro: 3,
} as const

export type PerfilName = keyof typeof PERFIL_IDS

export const PERFIL_OPTIONS = [
  { title: 'Administrador' as const, value: PERFIL_IDS.Administrador },
  { title: 'Produtor' as const, value: PERFIL_IDS.Produtor },
  { title: 'Financeiro' as const, value: PERFIL_IDS.Financeiro },
]
