# Module 5: Clipboard Monitoring - Detailed Implementation Plan

**Priority:** CRITICAL
**Total Estimated Time:** 14-18 hours
**Approach:** Incremental implementation with testing after each step

---

## 📋 Overview

Module 5 will make CopyGum actually functional by monitoring the clipboard and automatically saving content. We'll build this incrementally, testing each piece before moving to the next.

---

## 🎯 Step-by-Step Implementation Plan

### **Step 1: Basic Text Clipboard Monitoring** ⭐ START HERE
**Priority:** CRITICAL
**Time:** 2-3 hours
**Complexity:** Low

**Goal:** Get basic clipboard monitoring working for plain text only.

**Tasks:**
1. ✅ Create `src-tauri/src/clipboard_monitor.rs` file
2. ✅ Implement basic clipboard polling (every 500ms)
3. ✅ Detect when clipboard content changes (text only)
4. ✅ Log clipboard changes to console
5. ✅ Add start/stop monitoring commands
6. ✅ Test: Copy text and see it logged

**Files to Create/Modify:**
- `src-tauri/src/clipboard_monitor.rs` (NEW)
- `src-tauri/src/main.rs` (add module, register commands)

**Success Criteria:**
- [ ] Can start clipboard monitoring
- [ ] Console logs show clipboard text changes
- [ ] Can stop monitoring
- [ ] No crashes or memory leaks

**Dependencies:**
- `tauri-plugin-clipboard-manager` (already installed)

---

### **Step 2: Save Text to Database**
**Priority:** CRITICAL
**Time:** 2-3 hours
**Complexity:** Medium

**Goal:** Automatically save captured text to database.

**Tasks:**
1. ✅ Integrate `save_clipboard_item` command from Module 2
2. ✅ Auto-save captured text to database
3. ✅ Set default category as "text"
4. ✅ Generate timestamp
5. ✅ Test: Copy text and verify it appears in database

**Files to Modify:**
- `src-tauri/src/clipboard_monitor.rs` (add database save)

**Success Criteria:**
- [ ] Copied text automatically saves to database
- [ ] Can query clipboard_items table and see entries
- [ ] Timestamps are correct
- [ ] No duplicate saves for same content

---

### **Step 3: Debouncing & Duplicate Prevention**
**Priority:** HIGH
**Time:** 1-2 hours
**Complexity:** Low

**Goal:** Prevent saving the same content multiple times.

**Tasks:**
1. ✅ Implement debounce mechanism (1 second window)
2. ✅ Track last copied content hash
3. ✅ Skip save if content matches last copy within 1 second
4. ✅ Test: Copy same text twice quickly, should only save once

**Files to Modify:**
- `src-tauri/src/clipboard_monitor.rs` (add debounce logic)

**Success Criteria:**
- [ ] Same content copied rapidly only saves once
- [ ] Different content always saves
- [ ] After 1 second, same content can be saved again

**Implementation Approach:**
```rust
struct ClipboardMonitor {
    last_content: String,
    last_timestamp: i64,
    debounce_ms: i64, // 1000ms default
}

fn should_save(&self, new_content: &str, current_time: i64) -> bool {
    if new_content != self.last_content {
        return true; // Different content
    }

    // Same content, check time difference
    (current_time - self.last_timestamp) > self.debounce_ms
}
```

---

### **Step 4: Content Type Detection**
**Priority:** HIGH
**Time:** 3-4 hours
**Complexity:** Medium

**Goal:** Automatically detect content type (URL, email, color, code, etc.)

**Tasks:**
1. ✅ Create content type detection module
2. ✅ Implement regex patterns for:
   - URLs (http/https)
   - Emails
   - Hex colors (#RRGGBB)
   - Phone numbers
   - Numbers
   - Code (detect common patterns)
3. ✅ Set appropriate content_type when saving
4. ✅ Test each pattern type

**Files to Create/Modify:**
- `src-tauri/src/content_detector.rs` (NEW)
- `src-tauri/src/clipboard_monitor.rs` (use detector)
- `src-tauri/src/main.rs` (add module)

**Success Criteria:**
- [ ] URLs detected as "link" content type
- [ ] Emails detected as "email"
- [ ] Hex colors detected as "color"
- [ ] Plain text defaults to "text"

**Detection Priority Order:**
1. Hex color (#RRGGBB, #RGB)
2. URL (contains http:// or https://)
3. Email (contains @ with domain)
4. Phone (matches phone pattern)
5. Number (pure digits)
6. Code (has brackets, semicolons, etc.)
7. Default: text

---

### **Step 5: Auto-Categorization**
**Priority:** MEDIUM
**Time:** 2-3 hours
**Complexity:** Medium

**Goal:** Automatically assign categories based on content type.

**Tasks:**
1. ✅ Map content types to categories:
   - URL → "links"
   - Email → "email"
   - Hex color → "color"
   - Phone → "phone"
   - Number → "number"
   - Code patterns → "code"
   - Default → "text"
2. ✅ Update save to use detected category
3. ✅ Test: Copy different types, verify correct category

**Files to Modify:**
- `src-tauri/src/content_detector.rs` (return category)
- `src-tauri/src/clipboard_monitor.rs` (use category)

**Success Criteria:**
- [ ] URLs saved in "links" category
- [ ] Colors saved in "color" category
- [ ] Emails saved in "email" category
- [ ] Everything else in "text" category

---

### **Step 6: Image Clipboard Support** 🖼️
**Priority:** HIGH
**Time:** 4-5 hours
**Complexity:** High

**Goal:** Capture images from clipboard.

**Tasks:**
1. ✅ Detect when clipboard contains image
2. ✅ Extract image data from clipboard
3. ✅ Save image to filesystem (user data directory)
4. ✅ Generate unique filename (timestamp-based)
5. ✅ Save to database with image_path
6. ✅ Set category as "image"
7. ✅ Test: Copy image and verify it's saved

**Files to Create/Modify:**
- `src-tauri/src/image_handler.rs` (NEW)
- `src-tauri/src/clipboard_monitor.rs` (handle images)
- `src-tauri/src/main.rs` (add module)

**Success Criteria:**
- [ ] Can capture images from clipboard
- [ ] Images saved to filesystem
- [ ] Database has correct image_path
- [ ] Category set to "image"

**Dependencies:**
```toml
# Add to Cargo.toml
[dependencies]
image = "0.25"
```

**File Storage Path:**
```
{app_data_dir}/CopyGum/images/
└── {timestamp}_{random}.png
```

---

### **Step 7: Thumbnail Generation**
**Priority:** MEDIUM
**Time:** 2-3 hours
**Complexity:** Medium

**Goal:** Generate thumbnails for copied images.

**Tasks:**
1. ✅ Implement thumbnail generation (max 400x400)
2. ✅ Maintain aspect ratio
3. ✅ Save thumbnail to filesystem
4. ✅ Save thumbnail_path to database
5. ✅ Extract image dimensions (width, height)
6. ✅ Calculate file size
7. ✅ Test: Copy large image, verify thumbnail created

**Files to Modify:**
- `src-tauri/src/image_handler.rs` (add thumbnail generation)

**Success Criteria:**
- [ ] Thumbnails generated at max 400x400
- [ ] Aspect ratio preserved
- [ ] Database has image_width, image_height, image_size
- [ ] Thumbnail path saved

**Thumbnail Generation Logic:**
```rust
use image::imageops::FilterType;

fn generate_thumbnail(img: DynamicImage) -> DynamicImage {
    let max_dimension = 400;
    img.resize(max_dimension, max_dimension, FilterType::Lanczos3)
}
```

---

### **Step 8: Dominant Color Extraction**
**Priority:** LOW
**Time:** 2-3 hours
**Complexity:** Medium

**Goal:** Extract dominant color from images for visual categorization.

**Tasks:**
1. ✅ Implement dominant color extraction algorithm
2. ✅ Sample pixels from image
3. ✅ Calculate average/dominant color
4. ✅ Save as hex color to database
5. ✅ Test: Copy colorful image, verify color extracted

**Files to Modify:**
- `src-tauri/src/image_handler.rs` (add color extraction)

**Success Criteria:**
- [ ] Dominant color extracted and saved as hex (#RRGGBB)
- [ ] Color reasonably represents image
- [ ] Performance acceptable (<500ms)

**Algorithm Options:**
1. **Simple Average:** Average all pixel colors (fast)
2. **K-means:** Cluster colors and pick dominant (slower but better)
3. **Median Cut:** Good balance of speed and accuracy

**Recommended:** Start with simple average, can enhance later.

---

### **Step 9: App Detection (Source Application)**
**Priority:** MEDIUM
**Time:** 3-4 hours
**Complexity:** High (OS-specific)

**Goal:** Detect which application the copy came from.

**Tasks:**
1. ✅ Implement macOS app detection (NSWorkspace)
2. ✅ Implement Windows app detection (GetForegroundWindow)
3. ✅ Implement Linux app detection (X11/Wayland)
4. ✅ Extract app name
5. ✅ Extract app icon (optional)
6. ✅ Save app_name and app_icon to database
7. ✅ Test on each platform

**Files to Create/Modify:**
- `src-tauri/src/app_detector.rs` (NEW)
- `src-tauri/src/clipboard_monitor.rs` (use app detector)

**Success Criteria:**
- [ ] Can detect source app on current OS
- [ ] App name saved to database
- [ ] Gracefully handles when detection fails

**Platform-Specific Implementation:**

**macOS:**
```rust
#[cfg(target_os = "macos")]
fn get_active_app() -> Option<String> {
    // Use NSWorkspace.sharedWorkspace.frontmostApplication
    // Return app name
}
```

**Windows:**
```rust
#[cfg(target_os = "windows")]
fn get_active_app() -> Option<String> {
    // Use GetForegroundWindow + GetWindowText
    // Return app name
}
```

**Linux:**
```rust
#[cfg(target_os = "linux")]
fn get_active_app() -> Option<String> {
    // Use X11 or Wayland APIs
    // Return app name
}
```

---

### **Step 10: Privacy Mode Filtering** 🔒
**Priority:** LOW
**Time:** 2-3 hours
**Complexity:** Low

**Goal:** Skip saving sensitive content like passwords.

**Tasks:**
1. ✅ Implement pattern matching for sensitive content:
   - Passwords (common password patterns)
   - Credit card numbers
   - API keys
   - Private keys (SSH, GPG)
2. ✅ Add privacy mode toggle (setting)
3. ✅ Skip save if privacy filter matches
4. ✅ Test: Copy password-like string, verify not saved

**Files to Create/Modify:**
- `src-tauri/src/privacy_filter.rs` (NEW)
- `src-tauri/src/clipboard_monitor.rs` (use privacy filter)

**Success Criteria:**
- [ ] Credit card numbers not saved (when privacy mode on)
- [ ] API key patterns not saved
- [ ] Normal content still saved
- [ ] Privacy mode can be toggled

**Privacy Patterns:**
```rust
fn is_sensitive(content: &str) -> bool {
    // Credit card (13-19 digits with optional spaces/dashes)
    let cc_pattern = r"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b";

    // API keys (long alphanumeric strings)
    let api_pattern = r"\b[A-Za-z0-9]{32,}\b";

    // Password-like (many special chars + alphanumeric)
    // etc...
}
```

---

## 📊 Implementation Order Summary

**Phase 1: Core Functionality (CRITICAL)**
1. ✅ Step 1: Basic Text Monitoring (2-3h)
2. ✅ Step 2: Save to Database (2-3h)
3. ✅ Step 3: Debouncing (1-2h)

**Phase 2: Smart Detection (HIGH)**
4. ✅ Step 4: Content Type Detection (3-4h)
5. ✅ Step 5: Auto-Categorization (2-3h)

**Phase 3: Image Support (HIGH)**
6. ✅ Step 6: Image Capture (4-5h)
7. ✅ Step 7: Thumbnails (2-3h)

**Phase 4: Enhancements (MEDIUM/LOW)**
8. ✅ Step 8: Dominant Color (2-3h) - OPTIONAL
9. ✅ Step 9: App Detection (3-4h) - OPTIONAL
10. ✅ Step 10: Privacy Filter (2-3h) - OPTIONAL

---

## 🎯 Recommended Starting Point

**Start with Phase 1 (Steps 1-3)** to get basic functionality working:
- Step 1: Basic monitoring (2-3h)
- Step 2: Database save (2-3h)
- Step 3: Debouncing (1-2h)

**Total: 5-8 hours for core functionality**

After Phase 1 works, you'll have a functional clipboard manager that:
✅ Monitors clipboard changes
✅ Saves text automatically
✅ Prevents duplicates
✅ Works with existing database

---

## ✅ Success Criteria for Module 5 Complete

### Minimum Viable (Required):
- [ ] Text clipboard monitoring works
- [ ] Content auto-saves to database
- [ ] Debouncing prevents duplicates
- [ ] Content type detection works
- [ ] Auto-categorization works
- [ ] Image capture works
- [ ] Thumbnails generated

### Nice to Have (Optional):
- [ ] Dominant color extraction
- [ ] App detection (at least on one OS)
- [ ] Privacy filtering

---

## 🚀 Ready to Start?

**Recommended Approach:**
1. Start with Step 1 (Basic Text Monitoring)
2. Test thoroughly before moving to Step 2
3. After each step, verify everything still works
4. Commit changes after each successful step
5. If stuck, move to next step and return later

**Next Action:** Implement Step 1 - Basic Text Clipboard Monitoring

Would you like me to start implementing Step 1?
