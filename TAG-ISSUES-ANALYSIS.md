# 🔍 Tag Issues - Detailed Analysis

**Date:** 2025-11-22
**Reporter:** User
**Status:** 🔴 BUGS IDENTIFIED

---

## 🐛 Problem 1: Cannot Edit Tag Name

### User Report
"I can't edit name with that edit panel"

### Investigation Results

**File:** `ClipboardCard.svelte` line 557-568

```svelte
<CategoryEditPanel
  show={showEditPanel}
  categoryName={editName}
  categoryIcon={editIcon}
  categoryColor={editColor}
  showColorTab={editType === 'tag'}
  isNameEditable={editType === 'tag' || (editType === 'category' && editingIsCustomCategory)}
  panelTitle={editType === 'tag' ? 'Tag name' : 'Category name'}
  onSave={handleSaveEdit}
  onClose={handleCloseEditPanel}
/>
```

**Analysis:**
Line 563: `isNameEditable={editType === 'tag' || ...}`

This SHOULD make the name editable when `editType === 'tag'`.

**CategoryEditPanel.svelte** line 92-93:
```svelte
<input
  readonly={!isNameEditable}
  disabled={!isNameEditable}
```

### ✅ Code Looks Correct!

**Possible Issues:**
1. **Prop not updating:** `isNameEditable` might not be reactive
2. **Input focus issue:** Input field might need manual focus
3. **Event propagation:** Click events might be stopped before reaching input
4. **Disabled attribute precedence:** `disabled` overrides `readonly`

### 🎯 Most Likely Cause

The input has BOTH `readonly` and `disabled` attributes. When editing tags:
- `isNameEditable = true` (because `editType === 'tag'`)
- `readonly = !true = false` ✅ Should work
- `disabled = !true = false` ✅ Should work

**BUT:** The reactive update might not fire properly!

---

## 🐛 Problem 2: Tags Disappear After a Few Seconds

### User Report
"When I select any tag it shows on the card one time and disappear after a few second"

### Investigation Results

**File:** `ClipboardCard.svelte` line 215-224

```typescript
function handleToggleTag(tagName: string) {
  if (tags.includes(tagName)) {
    tags = tags.filter(t => t !== tagName);
  } else {
    tags = [...tags, tagName];
  }
  console.log('Tags updated:', tags);
  showTagDropdown = false; // Close dropdown after selecting
  // TODO: Update tags in database  ← ❌ NOT IMPLEMENTED
}
```

### 🔥 ROOT CAUSE FOUND!

**Line 223:** `// TODO: Update tags in database`

**What happens:**
1. User selects tag → `tags = [...tags, tagName]` → Tag added to LOCAL STATE
2. UI updates → Tag appears on card ✅
3. **A few seconds later** → Something re-fetches clipboard items from database
4. Database still has OLD tags (without the new one)
5. Component re-renders with database data → Tag DISAPPEARS ❌

**The Flow:**
```
User clicks tag
    ↓
tags array updated (LOCAL ONLY)
    ↓
UI shows tag (temporary)
    ↓
ClipboardStore refreshes from database
    ↓
Database doesn't have the tag
    ↓
tags array overwritten with DB data
    ↓
Tag DISAPPEARS ❌
```

### Evidence

**Line 511-528:** Tag rendering uses `{#key $allTags}`

```svelte
{#each tags as tag}
  {#key $allTags}  ← Reactively updates when allTags changes
    <span class="tag" ...>
      {tag}
    </span>
  {/key}
{/each}
```

This re-renders when `$allTags` store changes, which might trigger refresh from database.

---

## 📊 Problems Summary

| Problem | Root Cause | Severity | Impact |
|---------|------------|----------|--------|
| **Can't edit tag name** | Input field reactive update issue OR focus issue | Medium | Can't rename tags |
| **Tags disappear** | Not saved to database, overwritten on refresh | High | Tags don't persist |

---

## 🎯 Fix Plan

### Fix 1: Tag Name Editing

**Problem:** Input might not be focusable or reactive updates not working

**Solution Options:**

#### Option A: Remove `disabled` attribute
File: `CategoryEditPanel.svelte` line 92-93

```svelte
<!-- BEFORE -->
<input
  readonly={!isNameEditable}
  disabled={!isNameEditable}  ← Remove this
/>

<!-- AFTER -->
<input
  readonly={!isNameEditable}
/>
```

**Why:** `disabled` prevents ALL interaction, `readonly` still allows focus/select

#### Option B: Force input focus when panel opens
File: `CategoryEditPanel.svelte` line 74-81

```typescript
// Add this
import { tick } from 'svelte';
let inputElement: HTMLInputElement;

$: {
  if (show && isNameEditable) {
    tick().then(() => {
      inputElement?.focus();
      inputElement?.select();  // Also select all text
    });
  }
}
```

```svelte
<input
  bind:this={inputElement}  ← Add this
  type="text"
  ...
/>
```

#### Option C: Check if prop is actually updating
Add debug logging to verify:

```typescript
$: console.log('isNameEditable changed:', isNameEditable);
```

---

### Fix 2: Tag Persistence (CRITICAL)

**Problem:** Tags not saved to database

**Solution:** Implement database save in `handleToggleTag`

**File:** `ClipboardCard.svelte` line 215-224

**Current Code:**
```typescript
function handleToggleTag(tagName: string) {
  if (tags.includes(tagName)) {
    tags = tags.filter(t => t !== tagName);
  } else {
    tags = [...tags, tagName];
  }
  console.log('Tags updated:', tags);
  showTagDropdown = false;
  // TODO: Update tags in database  ← FIX THIS
}
```

**Fixed Code:**
```typescript
import { addTagToItem, removeTagFromItem } from '../../stores/clipboardStore';

export let itemId: number; // Need to pass this from parent

async function handleToggleTag(tagName: string) {
  const wasSelected = tags.includes(tagName);

  if (wasSelected) {
    // Remove tag
    tags = tags.filter(t => t !== tagName);

    // Save to database
    try {
      await removeTagFromItem(itemId, tagName);
      console.log('✅ Tag removed from database:', tagName);
    } catch (err) {
      console.error('❌ Failed to remove tag:', err);
      // Rollback local change
      tags = [...tags, tagName];
      showError('Failed to remove tag');
    }
  } else {
    // Add tag
    tags = [...tags, tagName];

    // Save to database
    try {
      await addTagToItem(itemId, tagName);
      console.log('✅ Tag added to database:', tagName);
    } catch (err) {
      console.error('❌ Failed to add tag:', err);
      // Rollback local change
      tags = tags.filter(t => t !== tagName);
      showError('Failed to add tag');
    }
  }

  showTagDropdown = false;
}
```

**Also need to implement in `database.ts`:**

```typescript
export async function addTagToItem(itemId: number, tagName: string): Promise<void> {
  const db = await getDb();

  // Get tag ID from tag name
  const tagResult = await db.select<{ id: number }[]>(
    'SELECT id FROM tags WHERE name = $1',
    [tagName]
  );

  if (tagResult.length === 0) {
    throw new Error(`Tag "${tagName}" not found`);
  }

  const tagId = tagResult[0].id;

  // Insert into item_tags junction table
  await db.execute(
    'INSERT OR IGNORE INTO item_tags (item_id, tag_id) VALUES ($1, $2)',
    [itemId, tagId]
  );
}

export async function removeTagFromItem(itemId: number, tagName: string): Promise<void> {
  const db = await getDb();

  // Get tag ID from tag name
  const tagResult = await db.select<{ id: number }[]>(
    'SELECT id FROM tags WHERE name = $1',
    [tagName]
  );

  if (tagResult.length === 0) {
    throw new Error(`Tag "${tagName}" not found`);
  }

  const tagId = tagResult[0].id;

  // Delete from item_tags junction table
  await db.execute(
    'DELETE FROM item_tags WHERE item_id = $1 AND tag_id = $2',
    [itemId, tagId]
  );
}
```

---

## 🧪 Testing Plan

### Test 1: Edit Tag Name
1. Click tag dropdown on a card
2. Hover over a tag → click edit icon
3. Edit panel should open
4. **Try to type in the name field**
5. **Expected:** Should be able to type ✅
6. **Current:** Cannot type ❌

### Test 2: Tag Persistence
1. Click tag dropdown on a card
2. Select a tag (e.g., "Urgent")
3. Tag appears on card ✅
4. **Wait 5 seconds**
5. **Expected:** Tag should stay ✅
6. **Current:** Tag disappears ❌

### Test 3: Tag Persistence After Refresh
1. Add tag to card
2. Close and reopen app
3. **Expected:** Tag should still be there ✅
4. **Current:** Tag is gone ❌

---

## 📝 Implementation Priority

### Priority 1: Fix Tag Persistence (HIGH)
**Why:** Tags don't persist at all - completely broken
**Files to modify:**
1. `src/lib/services/database.ts` - Add `addTagToItem` and `removeTagFromItem`
2. `src/lib/components/cards/ClipboardCard.svelte` - Update `handleToggleTag`
3. Pass `itemId` prop to ClipboardCard

**Estimated time:** 20-30 minutes

### Priority 2: Fix Tag Name Editing (MEDIUM)
**Why:** Can work around by deleting and recreating, but annoying
**Files to modify:**
1. `src/lib/components/categories/CategoryEditPanel.svelte` - Remove `disabled` or add focus

**Estimated time:** 5-10 minutes

---

## 🔍 Additional Issues Found

### Issue 3: Missing itemId prop
`ClipboardCard.svelte` doesn't receive `itemId` from parent!

**File:** `CardsContainer.svelte` needs to pass it:

```svelte
<ClipboardCard
  itemId={item.id}  ← ADD THIS
  content={item.content}
  ...
/>
```

### Issue 4: Tag name changes not reflected on cards
When you rename a tag globally (via edit), cards still show old tag name until refresh.

**Fix:** Update all cards' tags arrays when tag is renamed in store.

---

## 🎯 Root Causes Summary

1. **Tag editing:** Input field has `disabled` attribute OR reactive updates not working
2. **Tags disappear:** Database save not implemented, local changes overwritten on refresh
3. **Missing itemId:** Card component doesn't know which database row to update
4. **No error handling:** Failed operations silently fail

---

## ✅ Success Criteria

After fixes:
- [ ] Can edit tag names in edit panel
- [ ] Tags persist after selection
- [ ] Tags persist after app refresh
- [ ] Tags sync across all cards using same tag
- [ ] Error messages shown on failure
- [ ] No console errors

---

**Next Step:** Implement Priority 1 (Tag Persistence) first, then Priority 2 (Name Editing)

**Estimated Total Time:** 30-40 minutes
