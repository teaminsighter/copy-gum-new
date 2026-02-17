// Export/Import Service
// Handles exporting clipboard history to JSON/CSV and importing from JSON

import { invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import Database from '@tauri-apps/plugin-sql';
import type { ClipboardItem } from './database';

export interface ExportData {
  version: string;
  export_date: string;
  item_count: number;
  items: ClipboardItem[];
}

/**
 * Export clipboard history to JSON
 */
export async function exportToJSON(includeImages: boolean = false): Promise<void> {
  try {
    // Get all clipboard items from database
    const db = await Database.load('sqlite:copygum.db');
    const items: ClipboardItem[] = await db.select(
      'SELECT * FROM clipboard_items ORDER BY timestamp DESC'
    );

    // Prepare export data
    const exportData: ExportData = {
      version: '1.0',
      export_date: new Date().toISOString(),
      item_count: items.length,
      items: includeImages ? items : items.map(item => ({
        ...item,
        image_path: undefined,
        image_thumbnail: undefined,
        image_dominant_color: undefined,
      })),
    };

    // Show save dialog
    const filePath = await save({
      defaultPath: `copygum-export-${Date.now()}.json`,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });

    if (!filePath) return; // User cancelled

    // Save file
    const jsonData = JSON.stringify(exportData, null, 2);
    await invoke('save_export_file', {
      filePath,
      data: jsonData
    });

    console.log('✅ Exported to:', filePath);
  } catch (error) {
    console.error('Failed to export:', error);
    throw error;
  }
}

/**
 * Export clipboard history to CSV
 */
export async function exportToCSV(): Promise<void> {
  try {
    const db = await Database.load('sqlite:copygum.db');
    const items: ClipboardItem[] = await db.select(
      'SELECT * FROM clipboard_items ORDER BY timestamp DESC'
    );

    // Convert to CSV
    const headers = ['Content', 'Type', 'Category', 'Date', 'Pinned', 'Tags', 'App Name'];
    const rows = items.map(item => [
      `"${(item.content || '').replace(/"/g, '""').replace(/\n/g, ' ')}"`, // Escape quotes and newlines
      item.content_type,
      item.category,
      new Date(item.timestamp).toISOString(),
      item.is_pinned ? 'Yes' : 'No',
      item.tags || '',
      item.app_name || '',
    ]);

    const csvData = [
      headers.join(','),
      ...rows.map(row => row.join(','))
    ].join('\n');

    // Show save dialog
    const filePath = await save({
      defaultPath: `copygum-export-${Date.now()}.csv`,
      filters: [{
        name: 'CSV',
        extensions: ['csv']
      }]
    });

    if (!filePath) return; // User cancelled

    // Save file
    await invoke('save_export_file', {
      filePath,
      data: csvData
    });

    console.log('✅ Exported to CSV:', filePath);
  } catch (error) {
    console.error('Failed to export CSV:', error);
    throw error;
  }
}

/**
 * Import clipboard history from JSON
 */
export async function importFromJSON(): Promise<number> {
  try {
    // Show open dialog
    const filePath = await open({
      multiple: false,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });

    if (!filePath) return 0; // User cancelled

    // Read file
    const contents = await invoke<string>('read_import_file', { filePath });
    const importData: ExportData = JSON.parse(contents);

    // Validate version
    if (importData.version !== '1.0') {
      throw new Error(`Unsupported export version: ${importData.version}`);
    }

    // Import items into database
    const db = await Database.load('sqlite:copygum.db');
    let importedCount = 0;

    for (const item of importData.items) {
      try {
        // Insert item (will skip if duplicate based on content hash)
        await db.execute(
          `INSERT OR IGNORE INTO clipboard_items
           (content, content_type, category, timestamp, is_pinned, tags, app_name, app_icon,
            image_path, image_thumbnail, image_dominant_color, search_index)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
          [
            item.content,
            item.content_type,
            item.category,
            item.timestamp,
            item.is_pinned ? 1 : 0,
            item.tags || null,
            item.app_name || null,
            item.app_icon || null,
            item.image_path || null,
            item.image_thumbnail || null,
            item.image_dominant_color || null,
            item.content.toLowerCase(), // search_index
          ]
        );
        importedCount++;
      } catch (err) {
        console.warn(`Failed to import item:`, err);
      }
    }

    console.log(`✅ Imported ${importedCount} items from ${filePath}`);
    return importedCount;
  } catch (error) {
    console.error('Failed to import:', error);
    throw error;
  }
}

/**
 * Clear all clipboard history (for fresh import or reset)
 */
export async function clearAllHistory(): Promise<void> {
  const db = await Database.load('sqlite:copygum.db');
  await db.execute('DELETE FROM clipboard_items WHERE is_pinned = 0');
  console.log('✅ Cleared all unpinned history');
}
