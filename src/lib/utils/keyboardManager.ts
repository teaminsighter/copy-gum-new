// Keyboard Manager
// Centralized keyboard shortcut handling for CopyGum

export interface KeyboardShortcut {
  key: string;
  ctrl?: boolean;
  meta?: boolean;
  shift?: boolean;
  alt?: boolean;
  description: string;
  handler: (event: KeyboardEvent) => void;
  category: 'navigation' | 'actions' | 'search' | 'global';
}

class KeyboardManager {
  private shortcuts: Map<string, KeyboardShortcut> = new Map();
  private isEnabled = true;

  /**
   * Register a keyboard shortcut
   */
  register(shortcut: KeyboardShortcut): void {
    const key = this.getShortcutKey(shortcut);
    this.shortcuts.set(key, shortcut);
  }

  /**
   * Unregister a keyboard shortcut
   */
  unregister(shortcut: KeyboardShortcut): void {
    const key = this.getShortcutKey(shortcut);
    this.shortcuts.delete(key);
  }

  /**
   * Handle keyboard event
   */
  handleKeyPress(event: KeyboardEvent): boolean {
    if (!this.isEnabled) return false;

    // Skip if typing in input fields (unless it's a global shortcut)
    const target = event.target as HTMLElement;
    const isInput = target.tagName === 'INPUT' ||
                    target.tagName === 'TEXTAREA' ||
                    target.tagName === 'SELECT' ||
                    target.isContentEditable;

    const key = this.getEventKey(event);
    const shortcut = this.shortcuts.get(key);

    if (shortcut) {
      // Global shortcuts work everywhere, others skip inputs
      if (isInput && shortcut.category !== 'global' && shortcut.category !== 'search') {
        return false;
      }

      event.preventDefault();
      event.stopPropagation();
      shortcut.handler(event);
      return true;
    }

    return false;
  }

  /**
   * Enable/disable keyboard shortcuts
   */
  setEnabled(enabled: boolean): void {
    this.isEnabled = enabled;
  }

  /**
   * Get all registered shortcuts grouped by category
   */
  getShortcutsByCategory(): Record<string, KeyboardShortcut[]> {
    const grouped: Record<string, KeyboardShortcut[]> = {
      navigation: [],
      actions: [],
      search: [],
      global: []
    };

    this.shortcuts.forEach(shortcut => {
      grouped[shortcut.category].push(shortcut);
    });

    return grouped;
  }

  /**
   * Get readable shortcut label (e.g., "Cmd+F", "Ctrl+Shift+D")
   */
  getShortcutLabel(shortcut: KeyboardShortcut): string {
    const parts: string[] = [];

    if (shortcut.ctrl) parts.push('Ctrl');
    if (shortcut.meta) parts.push('Cmd');
    if (shortcut.alt) parts.push('Alt');
    if (shortcut.shift) parts.push('Shift');
    parts.push(shortcut.key.toUpperCase());

    return parts.join('+');
  }

  /**
   * Generate unique key for shortcut
   */
  private getShortcutKey(shortcut: KeyboardShortcut): string {
    const parts: string[] = [];
    if (shortcut.ctrl) parts.push('ctrl');
    if (shortcut.meta) parts.push('meta');
    if (shortcut.alt) parts.push('alt');
    if (shortcut.shift) parts.push('shift');
    parts.push(shortcut.key.toLowerCase());
    return parts.join('+');
  }

  /**
   * Generate key from keyboard event
   */
  private getEventKey(event: KeyboardEvent): string {
    const parts: string[] = [];
    if (event.ctrlKey) parts.push('ctrl');
    if (event.metaKey) parts.push('meta');
    if (event.altKey) parts.push('alt');
    if (event.shiftKey) parts.push('shift');
    parts.push(event.key.toLowerCase());
    return parts.join('+');
  }

  /**
   * Clear all shortcuts
   */
  clear(): void {
    this.shortcuts.clear();
  }
}

// Singleton instance
export const keyboardManager = new KeyboardManager();

/**
 * Initialize keyboard manager with default shortcuts
 */
export function initKeyboardManager(handlers: {
  onSearch?: () => void;
  onEscape?: () => void;
  onToggleWindow?: () => void;
  onNavigateUp?: () => void;
  onNavigateDown?: () => void;
  onNavigateLeft?: () => void;
  onNavigateRight?: () => void;
  onEnter?: () => void;
  onDelete?: () => void;
  onSelectAll?: () => void;
  onPin?: () => void;
  onCopy?: () => void;
  onShowHelp?: () => void;
}): void {
  // Global shortcuts
  if (handlers.onSearch) {
    keyboardManager.register({
      key: 'f',
      meta: true,
      description: 'Focus search',
      handler: handlers.onSearch,
      category: 'search'
    });
  }

  if (handlers.onEscape) {
    keyboardManager.register({
      key: 'Escape',
      description: 'Close window/panel',
      handler: handlers.onEscape,
      category: 'global'
    });
  }

  if (handlers.onToggleWindow) {
    keyboardManager.register({
      key: 'v',
      meta: true,
      shift: true,
      description: 'Toggle window',
      handler: handlers.onToggleWindow,
      category: 'global'
    });
  }

  // Navigation shortcuts
  if (handlers.onNavigateUp) {
    keyboardManager.register({
      key: 'ArrowUp',
      description: 'Navigate up',
      handler: handlers.onNavigateUp,
      category: 'navigation'
    });
  }

  if (handlers.onNavigateDown) {
    keyboardManager.register({
      key: 'ArrowDown',
      description: 'Navigate down',
      handler: handlers.onNavigateDown,
      category: 'navigation'
    });
  }

  if (handlers.onNavigateLeft) {
    keyboardManager.register({
      key: 'ArrowLeft',
      description: 'Navigate left',
      handler: handlers.onNavigateLeft,
      category: 'navigation'
    });
  }

  if (handlers.onNavigateRight) {
    keyboardManager.register({
      key: 'ArrowRight',
      description: 'Navigate right',
      handler: handlers.onNavigateRight,
      category: 'navigation'
    });
  }

  if (handlers.onEnter) {
    keyboardManager.register({
      key: 'Enter',
      description: 'Select/Copy item',
      handler: handlers.onEnter,
      category: 'navigation'
    });
  }

  // Action shortcuts
  if (handlers.onDelete) {
    keyboardManager.register({
      key: 'Delete',
      description: 'Delete selected item',
      handler: handlers.onDelete,
      category: 'actions'
    });

    keyboardManager.register({
      key: 'Backspace',
      description: 'Delete selected item',
      handler: handlers.onDelete,
      category: 'actions'
    });
  }

  if (handlers.onSelectAll) {
    keyboardManager.register({
      key: 'a',
      meta: true,
      description: 'Select all items',
      handler: handlers.onSelectAll,
      category: 'actions'
    });
  }

  if (handlers.onPin) {
    keyboardManager.register({
      key: 'p',
      meta: true,
      description: 'Pin/Unpin selected item',
      handler: handlers.onPin,
      category: 'actions'
    });
  }

  if (handlers.onCopy) {
    keyboardManager.register({
      key: 'c',
      meta: true,
      description: 'Copy selected item',
      handler: handlers.onCopy,
      category: 'actions'
    });
  }

  if (handlers.onShowHelp) {
    keyboardManager.register({
      key: '?',
      shift: true,
      description: 'Show keyboard shortcuts',
      handler: handlers.onShowHelp,
      category: 'global'
    });
  }
}

/**
 * Clean up keyboard manager
 */
export function cleanupKeyboardManager(): void {
  keyboardManager.clear();
}
