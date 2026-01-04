# Phase 3: Backend Integration Plan
**CopyGum - Clipboard Manager**

## Overview
Phase 3 focuses on connecting the completed UI (from Phase 2) with the Rust backend, implementing database operations, clipboard monitoring, and making the application fully functional.

---

## Current Status

### ✅ **Completed (Phase 1 & 2)**
- Complete UI implementation with all components
- Glassmorphism design system
- Category and tag management UI
- Card components (text, color, image)
- Settings UI
- Keyboard navigation (2D)
- Edit panels for categories/tags
- Toast notifications
- Sample data rendering

### 🔧 **Backend Stubs Exist**
- Database schema structures defined
- Tauri commands scaffolded but not implemented
- Window manager with global shortcut setup
- Plugins configured (SQL, clipboard, fs, shell, global-shortcut)

---

## Phase 3 Goals

1. **Implement database operations** (SQLite via tauri-plugin-sql)
2. **Connect frontend to backend** (Tauri invoke calls)
3. **Implement clipboard monitoring** (real-time capture)
4. **Image handling** (capture, thumbnail, storage)
5. **Settings persistence** (save user preferences)
6. **Auto-categorization** (content detection)
7. **Search functionality** (full-text search)
8. **Privacy mode** (blur sensitive content)
9. **Performance optimization** (pagination, caching)

---

## Implementation Roadmap

### **Module 1: Database Foundation** (Priority: CRITICAL)

#### 1.1 Database Schema & Migrations
**File:** `src-tauri/src/db/schema.rs` (create)

```rust
// Complete schema implementation based on design:
// - clipboard_items table
// - categories table (with custom categories)
// - tags table (with custom tags)
// - item_tags junction table
// - settings table
// - user_preferences table
```

**Tasks:**
- [ ] Create database schema SQL migrations
- [ ] Implement database initialization on app startup
- [ ] Add migration runner for schema updates
- [ ] Create indexes for search performance
- [ ] Add foreign key constraints

**Estimated Time:** 4-6 hours

---

#### 1.2 Database Connection & Pooling
**File:** `src-tauri/src/db/mod.rs` (update)

**Tasks:**
- [ ] Implement SQLite connection pool using `tauri-plugin-sql`
- [ ] Add connection lifecycle management
- [ ] Implement error handling and logging
- [ ] Create database helper utilities

**Estimated Time:** 2-3 hours

---

### **Module 2: Clipboard Item Operations** (Priority: CRITICAL)

#### 2.1 Save Clipboard Item
**File:** `src-tauri/src/db.rs` (update `save_clipboard_item`)

**Implementation:**
```rust
#[tauri::command]
pub async fn save_clipboard_item(
    db: State<'_, Database>,
    content: String,
    content_type: String,
    category: String,
    app_name: Option<String>,
    app_icon: Option<String>,
    image_path: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<i64, String>
```

**Features:**
- Insert item into `clipboard_items` table
- Auto-detect category if not provided
- Generate thumbnail for images
- Extract dominant color for color codes
- Associate tags via junction table
- Return inserted item ID

**Tasks:**
- [ ] Implement database insert
- [ ] Add content type detection
- [ ] Implement auto-categorization logic
- [ ] Add image processing (thumbnail, color extraction)
- [ ] Handle tag associations
- [ ] Add validation and sanitization

**Estimated Time:** 6-8 hours

---

#### 2.2 Get Clipboard Items
**File:** `src-tauri/src/db.rs` (update `get_clipboard_items`)

**Implementation:**
```rust
#[tauri::command]
pub async fn get_clipboard_items(
    db: State<'_, Database>,
    limit: Option<i32>,
    offset: Option<i32>,
    category: Option<String>,
    tags: Option<Vec<String>>,
    search_query: Option<String>,
) -> Result<Vec<ClipboardItem>, String>
```

**Features:**
- Pagination support (limit/offset)
- Filter by category
- Filter by tags (multiple)
- Search in content
- Order by timestamp DESC (pinned first)
- Exclude deleted items

**Tasks:**
- [ ] Implement query with filters
- [ ] Add pagination logic
- [ ] Join tags from junction table
- [ ] Implement pinned items priority
- [ ] Add search filtering
- [ ] Optimize query performance

**Estimated Time:** 4-6 hours

---

#### 2.3 Delete & Pin Operations
**Files:** `src-tauri/src/db.rs`

**Tasks:**
- [ ] Implement `delete_clipboard_item` (soft delete)
- [ ] Implement `toggle_pin` (update is_pinned)
- [ ] Implement `permanent_delete` (hard delete for cleanup)
- [ ] Add bulk operations (delete multiple, pin multiple)

**Estimated Time:** 2-3 hours

---

### **Module 3: Category Management** (Priority: HIGH)

#### 3.1 Category CRUD Operations
**File:** `src-tauri/src/db/categories.rs` (create)

**Commands to implement:**
```rust
create_custom_category(name, icon, color) -> Result<Category>
update_category(id, name, icon, color) -> Result<()>
delete_category(id) -> Result<()>
reorder_categories(category_ids: Vec<i64>) -> Result<()>
get_categories() -> Result<Vec<Category>>
```

**Tasks:**
- [ ] Create category CRUD operations
- [ ] Implement category reordering (drag-drop persistence)
- [ ] Add validation (prevent duplicate names)
- [ ] Implement cascading updates (when category deleted, reassign items)
- [ ] Update `get_categories` to return both default + custom

**Estimated Time:** 4-5 hours

---

#### 3.2 Connect Category UI to Backend
**Files:** `src/lib/stores/categoryStore.ts` (update)

**Tasks:**
- [ ] Replace mock data with Tauri invoke calls
- [ ] Implement `createCustomCategory` with backend call
- [ ] Implement `updateCustomCategory` with backend call
- [ ] Implement `deleteCustomCategory` with backend call
- [ ] Add error handling and loading states
- [ ] Update category list reactively after changes

**Estimated Time:** 3-4 hours

---

### **Module 4: Tag Management** (Priority: HIGH)

#### 4.1 Tag CRUD Operations
**File:** `src-tauri/src/db/tags.rs` (create)

**Commands to implement:**
```rust
create_tag(name, icon, color) -> Result<Tag>
update_tag(id, name, icon, color) -> Result<()>
delete_tag(id) -> Result<()>
get_tags() -> Result<Vec<Tag>>
add_tag_to_item(item_id, tag_id) -> Result<()>
remove_tag_from_item(item_id, tag_id) -> Result<()>
```

**Tasks:**
- [ ] Create tag CRUD operations
- [ ] Implement tag-item associations (junction table)
- [ ] Add validation (prevent duplicate tags)
- [ ] Implement cascading deletes (junction table cleanup)
- [ ] Return tag usage counts

**Estimated Time:** 4-5 hours

---

#### 4.2 Connect Tag UI to Backend
**Files:** `src/lib/stores/tagStore.ts` (update)

**Tasks:**
- [ ] Replace mock data with Tauri invoke calls
- [ ] Implement tag creation, update, delete
- [ ] Implement tag toggle on items
- [ ] Add error handling
- [ ] Update UI reactively

**Estimated Time:** 3-4 hours

---

### **Module 5: Clipboard Monitoring** (Priority: CRITICAL)

#### 5.1 Clipboard Watcher
**File:** `src-tauri/src/clipboard_monitor.rs` (create)

**Implementation:**
- Use `tauri-plugin-clipboard-manager` to monitor clipboard
- Detect changes in background
- Extract content and metadata
- Save to database automatically

**Features:**
- Real-time clipboard monitoring
- Debounce duplicate copies (same content within 1 second)
- Content type detection (text, image, color, email, url, etc.)
- App detection (get source application name/icon)
- Privacy mode filtering (skip passwords, credit cards)

**Tasks:**
- [ ] Implement clipboard polling/watching
- [ ] Add content type detection logic
- [ ] Implement app detection (macOS, Windows, Linux)
- [ ] Add debounce mechanism
- [ ] Integrate with database save
- [ ] Add privacy filters (optional skip patterns)

**Estimated Time:** 8-10 hours

---

#### 5.2 Image Clipboard Handling
**File:** `src-tauri/src/image_processor.rs` (create)

**Features:**
- Capture image from clipboard
- Generate thumbnail (400x400 max)
- Extract dominant color
- Save original to user data directory
- Save thumbnail as base64 or file

**Tasks:**
- [ ] Implement image capture from clipboard
- [ ] Create thumbnail generation (using `image` crate)
- [ ] Implement dominant color extraction
- [ ] Save images to filesystem
- [ ] Return image metadata to database

**Estimated Time:** 6-8 hours

---

### **Module 6: Search & Filter** (Priority: MEDIUM)

#### 6.1 Search Implementation
**File:** `src-tauri/src/db.rs` (update `search_clipboard`)

**Features:**
- Full-text search in content
- Search by category
- Search by tags
- Search by app name
- Date range filtering

**Implementation:**
```rust
#[tauri::command]
pub async fn search_clipboard(
    db: State<'_, Database>,
    query: String,
    filters: SearchFilters,
) -> Result<Vec<ClipboardItem>, String>
```

**Tasks:**
- [ ] Implement SQL full-text search (FTS5)
- [ ] Add category filtering
- [ ] Add tag filtering (AND/OR logic)
- [ ] Add date range filtering
- [ ] Optimize search performance (indexes)

**Estimated Time:** 5-6 hours

---

#### 6.2 Connect Search UI to Backend
**Files:** `src/lib/components/header/SearchBar.svelte` (update)

**Tasks:**
- [ ] Implement search input debouncing
- [ ] Call `search_clipboard` on input change
- [ ] Display search results dynamically
- [ ] Add loading state during search
- [ ] Clear search functionality

**Estimated Time:** 2-3 hours

---

### **Module 7: Settings Persistence** (Priority: MEDIUM)

#### 7.1 Settings Storage
**File:** `src-tauri/src/settings.rs` (create)

**Commands:**
```rust
get_settings() -> Result<Settings>
update_settings(settings: Settings) -> Result<()>
reset_settings() -> Result<Settings>
```

**Settings to persist:**
- Privacy mode enabled
- Sound effects enabled
- Launch at startup
- Notifications enabled
- Storage type (cloud/local)
- History limit (days)
- Auto-clear schedule
- License key & trial info

**Tasks:**
- [ ] Create settings table schema
- [ ] Implement get_settings command
- [ ] Implement update_settings command
- [ ] Add settings validation
- [ ] Create default settings

**Estimated Time:** 4-5 hours

---

#### 7.2 Connect Settings UI
**Files:** `src/lib/components/header/SettingsDropdown.svelte` (update)

**Tasks:**
- [ ] Load settings on mount
- [ ] Save settings on change
- [ ] Implement privacy mode blur
- [ ] Add sound effect triggers
- [ ] Implement launch at startup (OS-specific)

**Estimated Time:** 4-5 hours

---

### **Module 8: Auto-Categorization** (Priority: MEDIUM)

#### 8.1 Content Detection Logic
**File:** `src-tauri/src/categorizer.rs` (create)

**Detection Rules:**
- **Password**: Contains common password patterns
- **API Key**: Starts with sk-, pk-, Bearer, etc.
- **Email**: Valid email format
- **Phone**: Phone number patterns
- **Link**: Valid URL format
- **Code**: Contains code syntax patterns
- **Color**: Hex, RGB, HSL formats
- **Number**: Pure numeric content
- **Image**: Image data detected
- **Text**: Default fallback

**Tasks:**
- [ ] Implement regex patterns for each category
- [ ] Add priority-based detection (password > email > link > text)
- [ ] Create auto-categorization function
- [ ] Add confidence scoring
- [ ] Allow manual override

**Estimated Time:** 5-6 hours

---

### **Module 9: Performance Optimization** (Priority: LOW)

#### 9.1 Pagination & Virtual Scrolling

**Tasks:**
- [ ] Implement infinite scroll in CardsContainer
- [ ] Load items in batches (20-50 items)
- [ ] Add loading indicator
- [ ] Implement virtual scrolling for large lists

**Estimated Time:** 4-5 hours

---

#### 9.2 Caching & State Management

**Tasks:**
- [ ] Cache recently loaded items
- [ ] Invalidate cache on new items
- [ ] Optimize re-renders with Svelte stores
- [ ] Add debouncing for expensive operations

**Estimated Time:** 3-4 hours

---

### **Module 10: Privacy & Security** (Priority: HIGH)

#### 10.1 Privacy Mode
**Files:** Various UI components

**Tasks:**
- [ ] Implement blur filter for sensitive categories
- [ ] Add password/API key masking
- [ ] Privacy toggle in settings
- [ ] Exclude sensitive content from history (optional)

**Estimated Time:** 3-4 hours

---

#### 10.2 Data Encryption (Optional Enhancement)
**File:** `src-tauri/src/encryption.rs` (create)

**Tasks:**
- [ ] Encrypt sensitive clipboard items at rest
- [ ] Use OS keychain for encryption keys
- [ ] Decrypt on read

**Estimated Time:** 6-8 hours (if implemented)

---

### **Module 11: Testing & Polish** (Priority: MEDIUM)

#### 11.1 Testing

**Tasks:**
- [ ] Test clipboard monitoring on macOS
- [ ] Test database operations
- [ ] Test category/tag CRUD
- [ ] Test search functionality
- [ ] Test settings persistence
- [ ] Test keyboard shortcuts
- [ ] Test edge cases (empty clipboard, large images, etc.)

**Estimated Time:** 8-10 hours

---

#### 11.2 Error Handling & Logging

**Tasks:**
- [ ] Add comprehensive error messages
- [ ] Implement logging (to file)
- [ ] Add user-friendly error notifications
- [ ] Handle database errors gracefully

**Estimated Time:** 4-5 hours

---

## Priority Matrix

### CRITICAL (Do First)
1. Database schema & migrations
2. Save/Get clipboard items
3. Clipboard monitoring
4. Image handling

### HIGH (Do Next)
5. Category management backend + UI connection
6. Tag management backend + UI connection
7. Privacy mode
8. Settings persistence

### MEDIUM (Then Do)
9. Search implementation
10. Auto-categorization
11. Performance optimization
12. Testing

### LOW (Nice to Have)
13. Data encryption
14. Advanced analytics
15. Cloud sync

---

## Total Estimated Time

**Critical Tasks:** 24-32 hours
**High Priority:** 18-23 hours
**Medium Priority:** 25-32 hours
**Low Priority:** 6-8 hours (if implemented)

**Total:** 73-95 hours (approximately 2-3 weeks of full-time work)

---

## File Structure After Phase 3

```
src-tauri/src/
├── main.rs                      (updated with new commands)
├── db/
│   ├── mod.rs                   (database connection & pooling)
│   ├── schema.rs                (SQL schema & migrations)
│   ├── categories.rs            (category operations)
│   └── tags.rs                  (tag operations)
├── db.rs                        (clipboard item operations)
├── clipboard_monitor.rs         (NEW - clipboard watching)
├── image_processor.rs           (NEW - image handling)
├── categorizer.rs               (NEW - auto-categorization)
├── settings.rs                  (NEW - settings persistence)
├── encryption.rs                (OPTIONAL - data encryption)
└── window_manager.rs            (existing)

src/lib/
├── stores/
│   ├── categoryStore.ts         (updated - backend integration)
│   ├── tagStore.ts              (updated - backend integration)
│   ├── clipboardStore.ts        (NEW - clipboard items state)
│   └── settingsStore.ts         (NEW - settings state)
└── utils/
    └── tauri.ts                 (NEW - Tauri invoke helpers)
```

---

## Next Steps

1. **Review this plan** with the team
2. **Start with Module 1** (Database Foundation)
3. **Work through modules sequentially** following priority
4. **Test incrementally** after each module
5. **Iterate and refine** based on testing feedback

---

## Success Criteria

- ✅ Clipboard automatically monitored and saved
- ✅ Items persist across app restarts
- ✅ Categories and tags fully functional (CRUD)
- ✅ Search returns accurate results
- ✅ Settings persist and apply correctly
- ✅ Privacy mode works for sensitive content
- ✅ No performance issues with 1000+ items
- ✅ All UI interactions connected to backend
- ✅ Error handling graceful and user-friendly

---

**End of Phase 3 Plan**
