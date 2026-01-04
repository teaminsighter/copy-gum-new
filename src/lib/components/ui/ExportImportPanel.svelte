<script lang="ts">
  import { clipboardItems } from '../../stores/clipboardStore';
  import { settings } from '../../stores/settingsStore';
  import {
    exportToJSON,
    exportToCSV,
    importFromJSON,
    createFullBackup,
    restoreFromBackup,
    getExportSizeEstimate,
    validateImportedItems,
    type ExportData
  } from '../../utils/exportImport';
  import { saveClipboardItem } from '../../services/database';
  import { showSuccess, showError } from '../../stores/toastStore';

  let isExporting = false;
  let isImporting = false;
  let exportFormat: 'json' | 'csv' = 'json';
  let includeSettings = false;

  $: itemCount = $clipboardItems.length;
  $: estimatedSize = getExportSizeEstimate($clipboardItems);

  async function handleExport() {
    if (itemCount === 0) {
      showError('No items to export');
      return;
    }

    isExporting = true;
    try {
      if (exportFormat === 'json') {
        await exportToJSON($clipboardItems, includeSettings, includeSettings ? $settings : undefined);
        showSuccess(`Exported ${itemCount} items to JSON`);
      } else {
        await exportToCSV($clipboardItems);
        showSuccess(`Exported ${itemCount} items to CSV`);
      }
    } catch (error: any) {
      if (!error.message.includes('cancelled')) {
        showError(error.message || 'Export failed');
        console.error('Export error:', error);
      }
    } finally {
      isExporting = false;
    }
  }

  async function handleImport() {
    isImporting = true;
    try {
      const importData: ExportData = await importFromJSON();

      // Validate imported data
      if (!validateImportedItems(importData.items)) {
        throw new Error('Invalid data format in import file');
      }

      // Import items into database
      let importedCount = 0;
      for (const item of importData.items) {
        try {
          await saveClipboardItem(
            item.content,
            item.content_type,
            item.category || 'text'
          );
          importedCount++;
        } catch (error) {
          console.error('Failed to import item:', error);
        }
      }

      showSuccess(`Imported ${importedCount} of ${importData.items.length} items`);

      // Reload clipboard items to show imported data
      window.location.reload();
    } catch (error: any) {
      if (!error.message.includes('cancelled')) {
        showError(error.message || 'Import failed');
        console.error('Import error:', error);
      }
    } finally {
      isImporting = false;
    }
  }

  async function handleFullBackup() {
    isExporting = true;
    try {
      await createFullBackup($clipboardItems, $settings);
      showSuccess('Full backup created successfully');
    } catch (error: any) {
      if (!error.message.includes('cancelled')) {
        showError(error.message || 'Backup failed');
        console.error('Backup error:', error);
      }
    } finally {
      isExporting = false;
    }
  }

  async function handleRestore() {
    isImporting = true;
    try {
      const backupData = await restoreFromBackup();

      // Validate backup data
      if (!validateImportedItems(backupData.items)) {
        throw new Error('Invalid backup file format');
      }

      // Show confirmation
      const confirmed = confirm(
        `This will restore ${backupData.itemCount} items from ${new Date(backupData.exportDate).toLocaleDateString()}. Continue?`
      );

      if (!confirmed) {
        showError('Restore cancelled');
        return;
      }

      // Import items
      let importedCount = 0;
      for (const item of backupData.items) {
        try {
          await saveClipboardItem(
            item.content,
            item.content_type,
            item.category || 'text'
          );
          importedCount++;
        } catch (error) {
          console.error('Failed to restore item:', error);
        }
      }

      showSuccess(`Restored ${importedCount} items successfully`);

      // Reload to show restored data
      window.location.reload();
    } catch (error: any) {
      if (!error.message.includes('cancelled')) {
        showError(error.message || 'Restore failed');
        console.error('Restore error:', error);
      }
    } finally {
      isImporting = false;
    }
  }
</script>

<div class="export-import-panel">
  <!-- Export Section -->
  <div class="section">
    <h3 class="section-title">Export Data</h3>
    <p class="section-description">
      Export your clipboard history to a file
    </p>

    <div class="export-options">
      <div class="option-row">
        <label class="radio-option">
          <input type="radio" bind:group={exportFormat} value="json" />
          <span>JSON (Recommended)</span>
        </label>
        <label class="radio-option">
          <input type="radio" bind:group={exportFormat} value="csv" />
          <span>CSV (Spreadsheet)</span>
        </label>
      </div>

      {#if exportFormat === 'json'}
        <label class="checkbox-option">
          <input type="checkbox" bind:checked={includeSettings} />
          <span>Include settings</span>
        </label>
      {/if}
    </div>

    <div class="info-row">
      <span class="info-label">Items:</span>
      <span class="info-value">{itemCount}</span>
    </div>

    <div class="info-row">
      <span class="info-label">Estimated size:</span>
      <span class="info-value">{estimatedSize}</span>
    </div>

    <button
      class="action-button primary"
      on:click={handleExport}
      disabled={isExporting || itemCount === 0}
    >
      {#if isExporting}
        <span class="spinner"></span> Exporting...
      {:else}
        📤 Export {exportFormat.toUpperCase()}
      {/if}
    </button>
  </div>

  <!-- Import Section -->
  <div class="section">
    <h3 class="section-title">Import Data</h3>
    <p class="section-description">
      Import clipboard items from a JSON file
    </p>

    <button
      class="action-button secondary"
      on:click={handleImport}
      disabled={isImporting}
    >
      {#if isImporting}
        <span class="spinner"></span> Importing...
      {:else}
        📥 Import from JSON
      {/if}
    </button>
  </div>

  <!-- Backup Section -->
  <div class="section">
    <h3 class="section-title">Backup & Restore</h3>
    <p class="section-description">
      Create a full backup including all items and settings
    </p>

    <div class="button-row">
      <button
        class="action-button backup"
        on:click={handleFullBackup}
        disabled={isExporting || itemCount === 0}
      >
        {#if isExporting}
          <span class="spinner"></span>
        {:else}
          💾 Create Backup
        {/if}
      </button>

      <button
        class="action-button restore"
        on:click={handleRestore}
        disabled={isImporting}
      >
        {#if isImporting}
          <span class="spinner"></span>
        {:else}
          ♻️ Restore Backup
        {/if}
      </button>
    </div>
  </div>
</div>

<style>
  .export-import-panel {
    padding: 20px;
  }

  .section {
    margin-bottom: 32px;
    padding-bottom: 32px;
    border-bottom: 1px solid rgba(247, 228, 121, 0.2);
  }

  .section:last-child {
    border-bottom: none;
    margin-bottom: 0;
    padding-bottom: 0;
  }

  .section-title {
    font-size: 16px;
    font-weight: 600;
    color: #f7e479;
    margin: 0 0 8px 0;
  }

  .section-description {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.6);
    margin: 0 0 16px 0;
  }

  .export-options {
    margin-bottom: 16px;
  }

  .option-row {
    display: flex;
    gap: 16px;
    margin-bottom: 12px;
  }

  .radio-option,
  .checkbox-option {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.85);
    cursor: pointer;
  }

  .radio-option input,
  .checkbox-option input {
    cursor: pointer;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    padding: 8px 12px;
    background: rgba(247, 228, 121, 0.05);
    border-radius: 6px;
    margin-bottom: 8px;
    font-size: 13px;
  }

  .info-label {
    color: rgba(255, 255, 255, 0.6);
  }

  .info-value {
    color: #f7e479;
    font-weight: 600;
  }

  .action-button {
    width: 100%;
    padding: 12px 16px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border: none;
    margin-top: 12px;
  }

  .action-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .action-button.primary {
    background: linear-gradient(135deg, #f7e479 0%, #f59e0b 100%);
    color: #000;
    box-shadow: 0 4px 12px rgba(247, 228, 121, 0.3);
  }

  .action-button.primary:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(247, 228, 121, 0.4);
  }

  .action-button.secondary {
    background: rgba(247, 228, 121, 0.15);
    border: 1px solid rgba(247, 228, 121, 0.3);
    color: #f7e479;
  }

  .action-button.secondary:hover:not(:disabled) {
    background: rgba(247, 228, 121, 0.25);
    transform: translateY(-1px);
  }

  .button-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .action-button.backup {
    background: rgba(34, 197, 94, 0.15);
    border: 1px solid rgba(34, 197, 94, 0.3);
    color: rgba(34, 197, 94, 1);
  }

  .action-button.backup:hover:not(:disabled) {
    background: rgba(34, 197, 94, 0.25);
    transform: translateY(-1px);
  }

  .action-button.restore {
    background: rgba(59, 130, 246, 0.15);
    border: 1px solid rgba(59, 130, 246, 0.3);
    color: rgba(59, 130, 246, 1);
  }

  .action-button.restore:hover:not(:disabled) {
    background: rgba(59, 130, 246, 0.25);
    transform: translateY(-1px);
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: currentColor;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
