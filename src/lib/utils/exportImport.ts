// Export/Import Utilities
// Handles exporting and importing clipboard data in various formats

import type { ClipboardItem } from '../services/database';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';

export interface ExportData {
  version: string;
  exportDate: string;
  itemCount: number;
  items: ClipboardItem[];
  settings?: any;
}

/**
 * Export clipboard items to JSON
 */
export async function exportToJSON(
  items: ClipboardItem[],
  includeSettings: boolean = false,
  settings?: any
): Promise<void> {
  try {
    // Prepare export data
    const exportData: ExportData = {
      version: '1.0.0',
      exportDate: new Date().toISOString(),
      itemCount: items.length,
      items: items,
      ...(includeSettings && settings ? { settings } : {})
    };

    // Convert to JSON
    const jsonContent = JSON.stringify(exportData, null, 2);

    // Open save dialog
    const filePath = await save({
      defaultPath: `copygum-export-${new Date().toISOString().split('T')[0]}.json`,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });

    if (!filePath) {
      throw new Error('Save cancelled');
    }

    // Write file
    await writeTextFile(filePath, jsonContent);

    return;
  } catch (error) {
    throw new Error(`Failed to export to JSON: ${error}`);
  }
}

/**
 * Export clipboard items to CSV
 */
export async function exportToCSV(items: ClipboardItem[]): Promise<void> {
  try {
    // CSV headers
    const headers = [
      'ID',
      'Content',
      'Content Type',
      'Category',
      'Is Pinned',
      'Is Favorite',
      'Created At',
      'Tags'
    ];

    // Convert items to CSV rows
    const rows = items.map(item => {
      return [
        item.id?.toString() || '',
        `"${(item.content || '').replace(/"/g, '""')}"`, // Escape quotes
        item.content_type || '',
        item.category || '',
        item.is_pinned ? 'true' : 'false',
        item.is_favorite ? 'true' : 'false',
        item.created_at || '',
        `"${Array.isArray(item.tags) ? item.tags.join(', ') : (item.tags || '')}"` // Tags as comma-separated
      ];
    });

    // Combine headers and rows
    const csvContent = [
      headers.join(','),
      ...rows.map(row => row.join(','))
    ].join('\n');

    // Open save dialog
    const filePath = await save({
      defaultPath: `copygum-export-${new Date().toISOString().split('T')[0]}.csv`,
      filters: [{
        name: 'CSV',
        extensions: ['csv']
      }]
    });

    if (!filePath) {
      throw new Error('Save cancelled');
    }

    // Write file
    await writeTextFile(filePath, csvContent);

    return;
  } catch (error) {
    throw new Error(`Failed to export to CSV: ${error}`);
  }
}

/**
 * Import clipboard items from JSON
 */
export async function importFromJSON(): Promise<ExportData> {
  try {
    // Open file dialog
    const filePath = await open({
      multiple: false,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });

    if (!filePath) {
      throw new Error('Import cancelled');
    }

    // Read file
    const fileContent = await readTextFile(filePath as string);

    // Parse JSON
    const importData: ExportData = JSON.parse(fileContent);

    // Validate data structure
    if (!importData.version || !importData.items || !Array.isArray(importData.items)) {
      throw new Error('Invalid export file format');
    }

    return importData;
  } catch (error) {
    if (error instanceof Error && error.message === 'Import cancelled') {
      throw error;
    }
    throw new Error(`Failed to import from JSON: ${error}`);
  }
}

/**
 * Create a full backup (export everything)
 */
export async function createFullBackup(
  items: ClipboardItem[],
  settings: any
): Promise<void> {
  try {
    const exportData: ExportData = {
      version: '1.0.0',
      exportDate: new Date().toISOString(),
      itemCount: items.length,
      items: items,
      settings: settings
    };

    const jsonContent = JSON.stringify(exportData, null, 2);

    const filePath = await save({
      defaultPath: `copygum-full-backup-${new Date().toISOString().split('T')[0]}.json`,
      filters: [{
        name: 'CopyGum Backup',
        extensions: ['json']
      }]
    });

    if (!filePath) {
      throw new Error('Backup cancelled');
    }

    await writeTextFile(filePath, jsonContent);

    return;
  } catch (error) {
    throw new Error(`Failed to create backup: ${error}`);
  }
}

/**
 * Restore from a full backup
 */
export async function restoreFromBackup(): Promise<ExportData> {
  try {
    const filePath = await open({
      multiple: false,
      filters: [{
        name: 'CopyGum Backup',
        extensions: ['json']
      }]
    });

    if (!filePath) {
      throw new Error('Restore cancelled');
    }

    const fileContent = await readTextFile(filePath as string);
    const backupData: ExportData = JSON.parse(fileContent);

    // Validate backup
    if (!backupData.version || !backupData.items) {
      throw new Error('Invalid backup file format');
    }

    return backupData;
  } catch (error) {
    if (error instanceof Error && error.message === 'Restore cancelled') {
      throw error;
    }
    throw new Error(`Failed to restore backup: ${error}`);
  }
}

/**
 * Get file size estimate for export
 */
export function getExportSizeEstimate(items: ClipboardItem[]): string {
  const jsonString = JSON.stringify({ items });
  const sizeInBytes = new Blob([jsonString]).size;

  if (sizeInBytes < 1024) {
    return `${sizeInBytes} B`;
  } else if (sizeInBytes < 1024 * 1024) {
    return `${(sizeInBytes / 1024).toFixed(2)} KB`;
  } else {
    return `${(sizeInBytes / (1024 * 1024)).toFixed(2)} MB`;
  }
}

/**
 * Validate imported items before saving to database
 */
export function validateImportedItems(items: any[]): boolean {
  if (!Array.isArray(items)) return false;

  return items.every(item => {
    return (
      typeof item === 'object' &&
      item !== null &&
      typeof item.content === 'string' &&
      typeof item.content_type === 'string'
    );
  });
}
