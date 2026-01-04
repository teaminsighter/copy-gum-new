-- CopyGum Database Schema
-- Reference: Development Plan Phase 1.2

-- Main clipboard items table
CREATE TABLE IF NOT EXISTS clipboard_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    content_type TEXT DEFAULT 'text',  -- text, image, color
    category TEXT DEFAULT 'all',       -- all, email, links, code, etc.

    -- App source info
    app_name TEXT,                     -- "VS Code", "Chrome"
    app_icon TEXT,                     -- emoji or path to icon
    app_bundle_id TEXT,                -- com.google.Chrome

    -- Image specific fields
    is_image BOOLEAN DEFAULT 0,
    image_path TEXT,                   -- path to original image
    image_thumbnail BLOB,              -- WebP thumbnail (base64)
    image_dominant_color TEXT,         -- rgb(255,87,51)
    image_blurhash TEXT,               -- blur placeholder
    image_width INTEGER,
    image_height INTEGER,
    image_size INTEGER,                -- bytes

    -- Metadata
    timestamp INTEGER NOT NULL,        -- Unix timestamp
    char_count INTEGER,
    is_pinned BOOLEAN DEFAULT 0,
    tags TEXT,                         -- JSON: ["work", "urgent"]

    -- Status
    is_deleted BOOLEAN DEFAULT 0,
    deleted_at INTEGER,

    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- Categories table (from preview.html lines 2055-2220)
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    icon TEXT NOT NULL,                -- emoji
    color TEXT,                        -- gradient or solid
    is_custom BOOLEAN DEFAULT 0,       -- user created?
    sort_order INTEGER,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- Default categories (from preview.html)
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

-- Tags table (from preview.html lines 4530-4537)
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    icon TEXT NOT NULL,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- Default tags
INSERT OR IGNORE INTO tags (name, icon) VALUES
('work', '💼'),
('personal', '👤'),
('urgent', '🔥'),
('important', '⭐'),
('project', '📁'),
('meeting', '📅'),
('ideas', '💡');

-- Settings table
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Default settings
INSERT OR IGNORE INTO settings (key, value) VALUES
('storage_type', 'local'),
('storage_path', ''),
('theme', 'gradient-purple'),
('auto_delete_days', '7'),
('image_thumbnail_size', '200'),
('image_quality', 'medium'),
('shortcut_open_panel', 'CmdOrCtrl+Shift+V'),
('shortcut_search', 'CmdOrCtrl+F');

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_timestamp ON clipboard_items(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_category ON clipboard_items(category);
CREATE INDEX IF NOT EXISTS idx_is_deleted ON clipboard_items(is_deleted);
CREATE INDEX IF NOT EXISTS idx_is_pinned ON clipboard_items(is_pinned);
