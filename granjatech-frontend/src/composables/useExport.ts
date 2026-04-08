import { jsPDF } from 'jspdf'
import autoTable from 'jspdf-autotable'
import * as XLSX from 'xlsx'

export interface ExportColumn {
  header: string
  key: string
  width?: number
}

export function useExport() {
  function exportToPdf(title: string, columns: ExportColumn[], data: Record<string, any>[]) {
    const doc = new jsPDF()
    doc.setFontSize(16)
    doc.text(title, 14, 20)
    doc.setFontSize(10)
    doc.text(`Gerado em: ${new Date().toLocaleDateString('pt-BR')}`, 14, 28)

    autoTable(doc, {
      startY: 35,
      head: [columns.map(c => c.header)],
      body: data.map(row => columns.map(c => String(row[c.key] ?? ''))),
    })

    doc.save(`${title.toLowerCase().replace(/\s+/g, '-')}.pdf`)
  }

  function exportToExcel(filename: string, columns: ExportColumn[], data: Record<string, any>[]) {
    const wsData = [
      columns.map(c => c.header),
      ...data.map(row => columns.map(c => row[c.key] ?? '')),
    ]
    const ws = XLSX.utils.aoa_to_sheet(wsData)
    const wb = XLSX.utils.book_new()
    XLSX.utils.book_append_sheet(wb, ws, 'Dados')
    XLSX.writeFile(wb, `${filename}.xlsx`)
  }

  return { exportToPdf, exportToExcel }
}
