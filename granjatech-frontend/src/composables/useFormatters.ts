/**
 * Shared formatting utilities for dates and currency.
 * Centralizes locale-aware formatting that was previously duplicated across views.
 */

export function useFormatters() {
  /**
   * Format a date string to pt-BR locale date (DD/MM/YYYY).
   * Returns '-' for falsy inputs.
   */
  function formatDate(dateStr: string): string {
    if (!dateStr) return '-'
    return new Date(dateStr).toLocaleDateString('pt-BR')
  }

  /**
   * Format a date string to pt-BR locale date+time (DD/MM/YYYY HH:mm:ss).
   * Falls back to raw string on parse error.
   */
  function formatDateTime(dateStr: string): string {
    if (!dateStr) return '-'
    try {
      return new Date(dateStr).toLocaleString('pt-BR')
    } catch {
      return dateStr
    }
  }

  /**
   * Format a numeric value as BRL currency (R$ X.XXX,XX).
   * Returns 'R$ 0,00' for undefined/null/NaN values.
   */
  function formatCurrency(value: number | undefined | null): string {
    const num = Number(value)
    if (isNaN(num)) return 'R$ 0,00'
    return num.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' })
  }

  return { formatDate, formatDateTime, formatCurrency }
}
