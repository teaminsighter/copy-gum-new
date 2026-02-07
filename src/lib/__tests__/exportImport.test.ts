import { describe, it, expect } from 'vitest';
import { getExportSizeEstimate, validateImportedItems } from '../utils/exportImport';

describe('exportImport utilities', () => {
  describe('getExportSizeEstimate', () => {
    it('returns bytes for very small items', () => {
      const items: any[] = [];
      const estimate = getExportSizeEstimate(items);

      // Empty array should be small
      expect(estimate).toMatch(/B$/);
    });

    it('returns KB for moderately sized items', () => {
      // Create items that will be > 1KB but < 1MB
      const items = Array.from({ length: 50 }, (_, i) => ({
        id: i,
        content: 'x'.repeat(50),
        content_type: 'text',
        category: 'text',
        is_pinned: false,
        is_favorite: false,
        created_at: new Date().toISOString(),
        tags: [],
      }));

      const estimate = getExportSizeEstimate(items);
      expect(estimate).toMatch(/KB$/);
    });

    it('returns MB for large items', () => {
      // Create items that will be > 1MB (need ~1.1MB to be sure)
      const items = Array.from({ length: 100 }, (_, i) => ({
        id: i,
        content: 'x'.repeat(12000), // 12KB per item * 100 = 1.2MB
        content_type: 'text',
        category: 'text',
        is_pinned: false,
        is_favorite: false,
        created_at: new Date().toISOString(),
        tags: [],
      }));

      const estimate = getExportSizeEstimate(items);
      expect(estimate).toMatch(/MB$/);
    });

    it('returns correctly formatted size with decimals', () => {
      const items = Array.from({ length: 100 }, (_, i) => ({
        id: i,
        content: 'x'.repeat(100),
        content_type: 'text',
      }));

      const estimate = getExportSizeEstimate(items);
      // Should match pattern like "12.34 KB" or "1.23 MB"
      expect(estimate).toMatch(/^\d+(\.\d{1,2})?\s*(B|KB|MB)$/);
    });
  });

  describe('validateImportedItems', () => {
    it('returns true for valid items array', () => {
      const validItems = [
        { content: 'Hello World', content_type: 'text' },
        { content: 'https://example.com', content_type: 'links' },
        { content: '#FF5733', content_type: 'color' },
      ];

      expect(validateImportedItems(validItems)).toBe(true);
    });

    it('returns false for non-array input', () => {
      expect(validateImportedItems('not an array' as any)).toBe(false);
      expect(validateImportedItems({} as any)).toBe(false);
      expect(validateImportedItems(null as any)).toBe(false);
      expect(validateImportedItems(undefined as any)).toBe(false);
    });

    it('returns true for empty array', () => {
      expect(validateImportedItems([])).toBe(true);
    });

    it('returns false if any item is missing content', () => {
      const invalidItems = [
        { content: 'Valid', content_type: 'text' },
        { content_type: 'text' }, // missing content
      ];

      expect(validateImportedItems(invalidItems)).toBe(false);
    });

    it('returns false if any item is missing content_type', () => {
      const invalidItems = [
        { content: 'Valid', content_type: 'text' },
        { content: 'Missing type' }, // missing content_type
      ];

      expect(validateImportedItems(invalidItems)).toBe(false);
    });

    it('returns false if content is not a string', () => {
      const invalidItems = [
        { content: 123, content_type: 'number' },
      ];

      expect(validateImportedItems(invalidItems)).toBe(false);
    });

    it('returns false if content_type is not a string', () => {
      const invalidItems = [
        { content: 'Valid', content_type: 123 },
      ];

      expect(validateImportedItems(invalidItems)).toBe(false);
    });

    it('returns false if item is null', () => {
      const invalidItems = [
        { content: 'Valid', content_type: 'text' },
        null,
      ];

      expect(validateImportedItems(invalidItems)).toBe(false);
    });

    it('returns false if item is not an object', () => {
      const invalidItems = [
        { content: 'Valid', content_type: 'text' },
        'string item',
      ];

      expect(validateImportedItems(invalidItems)).toBe(false);
    });

    it('accepts items with additional properties', () => {
      const validItems = [
        {
          content: 'Hello',
          content_type: 'text',
          id: 1,
          category: 'text',
          is_pinned: true,
          created_at: '2024-01-15T10:00:00Z',
          tags: ['tag1', 'tag2'],
          extra_field: 'ignored',
        },
      ];

      expect(validateImportedItems(validItems)).toBe(true);
    });
  });
});
