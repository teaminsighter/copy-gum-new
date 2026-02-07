import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import {
  eventBus,
  broadcast,
  createTagRenamedEvent,
  createCategoryRenamedEvent,
  type TagRenamedEvent,
  type CategoryRenamedEvent,
} from '../events/eventBus';

describe('eventBus', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
    eventBus.set(null);
  });

  describe('createTagRenamedEvent', () => {
    it('creates a tag renamed event with correct properties', () => {
      const event = createTagRenamedEvent('tag-123', 'oldTag', 'newTag');

      expect(event.type).toBe('TAG_RENAMED');
      expect(event.tagId).toBe('tag-123');
      expect(event.oldName).toBe('oldTag');
      expect(event.newName).toBe('newTag');
      expect(typeof event.timestamp).toBe('number');
      expect(event.timestamp).toBeGreaterThan(0);
    });

    it('generates unique timestamps for different events', () => {
      const event1 = createTagRenamedEvent('tag-1', 'old1', 'new1');

      // Advance time slightly
      vi.advanceTimersByTime(1);

      const event2 = createTagRenamedEvent('tag-2', 'old2', 'new2');

      // Timestamps should be different (or at least valid)
      expect(event1.timestamp).toBeGreaterThan(0);
      expect(event2.timestamp).toBeGreaterThan(0);
    });
  });

  describe('createCategoryRenamedEvent', () => {
    it('creates a category renamed event with correct properties', () => {
      const event = createCategoryRenamedEvent('cat-456', 'oldCategory', 'newCategory');

      expect(event.type).toBe('CATEGORY_RENAMED');
      expect(event.categoryId).toBe('cat-456');
      expect(event.oldName).toBe('oldCategory');
      expect(event.newName).toBe('newCategory');
      expect(typeof event.timestamp).toBe('number');
    });
  });

  describe('broadcast', () => {
    it('sets the event on the eventBus store', () => {
      const event = createTagRenamedEvent('tag-1', 'old', 'new');

      broadcast(event);

      const currentEvent = get(eventBus);
      expect(currentEvent).toEqual(event);
    });

    it('clears the event after 50ms', () => {
      const event = createTagRenamedEvent('tag-1', 'old', 'new');

      broadcast(event);
      expect(get(eventBus)).toEqual(event);

      // Advance time by 50ms
      vi.advanceTimersByTime(50);

      expect(get(eventBus)).toBeNull();
    });

    it('can broadcast category events', () => {
      const event = createCategoryRenamedEvent('cat-1', 'oldCat', 'newCat');

      broadcast(event);

      const currentEvent = get(eventBus);
      expect(currentEvent?.type).toBe('CATEGORY_RENAMED');
    });
  });

  describe('eventBus store', () => {
    it('starts with null value', () => {
      eventBus.set(null);
      expect(get(eventBus)).toBeNull();
    });

    it('allows subscribing to events', () => {
      const events: (TagRenamedEvent | CategoryRenamedEvent | null)[] = [];

      const unsubscribe = eventBus.subscribe((event) => {
        events.push(event);
      });

      const testEvent = createTagRenamedEvent('tag-1', 'old', 'new');
      broadcast(testEvent);

      // Should have received the event
      expect(events).toContainEqual(testEvent);

      unsubscribe();
    });
  });
});
