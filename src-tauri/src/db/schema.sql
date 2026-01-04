-- CopyGum Database Schema
-- SQLite Database for Clipboard Manager

-- ============================================
-- CATEGORIES TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    icon TEXT NOT NULL,
    color TEXT,
    is_custom BOOLEAN NOT NULL DEFAULT 0,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Insert default categories
INSERT OR IGNORE INTO categories (name, icon, color, is_custom, sort_order) VALUES
('all', '📌', 'rgba(50, 50, 50, 0.6)', 0, 0),
('password', '🔐', 'rgba(50, 50, 50, 0.6)', 0, 1),
('apikey', '🔑', 'rgba(50, 50, 50, 0.6)', 0, 2),
('private', '🔒', 'rgba(50, 50, 50, 0.6)', 0, 3),
('email', '📧', 'rgba(50, 50, 50, 0.6)', 0, 4),
('phone', '📱', 'rgba(50, 50, 50, 0.6)', 0, 5),
('links', '🔗', 'rgba(50, 50, 50, 0.6)', 0, 6),
('code', '💻', 'rgba(50, 50, 50, 0.6)', 0, 7),
('color', '🎨', 'rgba(50, 50, 50, 0.6)', 0, 8),
('image', '🖼️', 'rgba(50, 50, 50, 0.6)', 0, 9),
('number', '🔢', 'rgba(50, 50, 50, 0.6)', 0, 10),
('text', '📝', 'rgba(50, 50, 50, 0.6)', 0, 11);

-- ============================================
-- TAGS TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    icon TEXT,
    color TEXT,
    is_custom BOOLEAN NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Insert default tags
INSERT OR IGNORE INTO tags (name, icon, color, is_custom) VALUES
('Work', '💼', 'rgba(59, 130, 246, 0.6)', 0),
('Personal', '👤', 'rgba(139, 79, 155, 0.6)', 0),
('Important', '⭐', 'rgba(247, 228, 121, 0.6)', 0),
('Project', '🎯', 'rgba(34, 197, 94, 0.6)', 0);

-- ============================================
-- CLIPBOARD ITEMS TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS clipboard_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL DEFAULT 'text',
    category TEXT NOT NULL DEFAULT 'text',

    -- App metadata
    app_name TEXT,
    app_icon TEXT,

    -- Image metadata
    is_image BOOLEAN NOT NULL DEFAULT 0,
    image_path TEXT,
    image_thumbnail TEXT, -- Base64 or file path
    image_width INTEGER,
    image_height INTEGER,
    image_size INTEGER, -- File size in bytes
    image_dominant_color TEXT,

    -- Timestamps
    timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),

    -- Flags
    is_pinned BOOLEAN NOT NULL DEFAULT 0,
    is_favorite BOOLEAN NOT NULL DEFAULT 0,
    is_deleted BOOLEAN NOT NULL DEFAULT 0,
    deleted_at INTEGER,

    -- Search
    search_text TEXT, -- Denormalized search field

    FOREIGN KEY (category) REFERENCES categories(name) ON DELETE SET DEFAULT
);

-- ============================================
-- ITEM_TAGS JUNCTION TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS item_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),

    UNIQUE(item_id, tag_id),
    FOREIGN KEY (item_id) REFERENCES clipboard_items(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- ============================================
-- SETTINGS TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY CHECK (id = 1), -- Only one row allowed

    -- Privacy & Security
    privacy_mode_enabled BOOLEAN NOT NULL DEFAULT 0,
    blur_sensitive_content BOOLEAN NOT NULL DEFAULT 1,

    -- UI Preferences
    sound_effects_enabled BOOLEAN NOT NULL DEFAULT 1,
    notifications_enabled BOOLEAN NOT NULL DEFAULT 1,
    theme TEXT NOT NULL DEFAULT 'dark',

    -- System Integration
    launch_at_startup BOOLEAN NOT NULL DEFAULT 0,
    global_shortcut TEXT NOT NULL DEFAULT 'CommandOrControl+Shift+V',

    -- Storage
    storage_type TEXT NOT NULL DEFAULT 'local', -- 'local' or 'cloud'
    history_limit_days INTEGER NOT NULL DEFAULT 30,
    auto_clear_enabled BOOLEAN NOT NULL DEFAULT 0,
    auto_clear_days INTEGER NOT NULL DEFAULT 30,

    -- License
    license_key TEXT,
    license_type TEXT NOT NULL DEFAULT 'trial', -- 'trial', 'basic', 'pro'
    trial_start_date INTEGER,
    trial_days_remaining INTEGER NOT NULL DEFAULT 7,

    -- Clipboard Monitoring
    monitor_clipboard BOOLEAN NOT NULL DEFAULT 1,
    ignore_duplicates BOOLEAN NOT NULL DEFAULT 1,
    duplicate_threshold_seconds INTEGER NOT NULL DEFAULT 1,

    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Insert default settings
INSERT OR IGNORE INTO settings (id) VALUES (1);

-- ============================================
-- USER PREFERENCES TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS user_preferences (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    value TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- ============================================
-- INDEXES FOR PERFORMANCE
-- ============================================

-- Clipboard Items Indexes
CREATE INDEX IF NOT EXISTS idx_clipboard_items_timestamp ON clipboard_items(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_clipboard_items_category ON clipboard_items(category);
CREATE INDEX IF NOT EXISTS idx_clipboard_items_pinned ON clipboard_items(is_pinned);
CREATE INDEX IF NOT EXISTS idx_clipboard_items_deleted ON clipboard_items(is_deleted);
CREATE INDEX IF NOT EXISTS idx_clipboard_items_content_type ON clipboard_items(content_type);
CREATE INDEX IF NOT EXISTS idx_clipboard_items_created_at ON clipboard_items(created_at DESC);

-- Full-text search index
CREATE VIRTUAL TABLE IF NOT EXISTS clipboard_items_fts USING fts5(
    content,
    app_name,
    category,
    content='clipboard_items',
    content_rowid='id'
);

-- Triggers to keep FTS index in sync
CREATE TRIGGER IF NOT EXISTS clipboard_items_fts_insert AFTER INSERT ON clipboard_items BEGIN
    INSERT INTO clipboard_items_fts(rowid, content, app_name, category)
    VALUES (new.id, new.content, new.app_name, new.category);
END;

CREATE TRIGGER IF NOT EXISTS clipboard_items_fts_delete AFTER DELETE ON clipboard_items BEGIN
    DELETE FROM clipboard_items_fts WHERE rowid = old.id;
END;

CREATE TRIGGER IF NOT EXISTS clipboard_items_fts_update AFTER UPDATE ON clipboard_items BEGIN
    DELETE FROM clipboard_items_fts WHERE rowid = old.id;
    INSERT INTO clipboard_items_fts(rowid, content, app_name, category)
    VALUES (new.id, new.content, new.app_name, new.category);
END;

-- Item Tags Indexes
CREATE INDEX IF NOT EXISTS idx_item_tags_item_id ON item_tags(item_id);
CREATE INDEX IF NOT EXISTS idx_item_tags_tag_id ON item_tags(tag_id);

-- Categories Index
CREATE INDEX IF NOT EXISTS idx_categories_sort_order ON categories(sort_order);
CREATE INDEX IF NOT EXISTS idx_categories_name ON categories(name);

-- Tags Index
CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);

-- ============================================
-- VIEWS FOR COMMON QUERIES
-- ============================================

-- View for active (non-deleted) clipboard items
CREATE VIEW IF NOT EXISTS active_clipboard_items AS
SELECT * FROM clipboard_items
WHERE is_deleted = 0
ORDER BY is_pinned DESC, timestamp DESC;

-- View for clipboard items with tags
CREATE VIEW IF NOT EXISTS clipboard_items_with_tags AS
SELECT
    ci.*,
    GROUP_CONCAT(t.name, ',') as tag_names,
    GROUP_CONCAT(t.icon, ',') as tag_icons,
    GROUP_CONCAT(t.color, ',') as tag_colors
FROM clipboard_items ci
LEFT JOIN item_tags it ON ci.id = it.item_id
LEFT JOIN tags t ON it.tag_id = t.id
WHERE ci.is_deleted = 0
GROUP BY ci.id
ORDER BY ci.is_pinned DESC, ci.timestamp DESC;
