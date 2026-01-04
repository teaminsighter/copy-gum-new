// Click Outside utility for Svelte
// Detects clicks outside an element and triggers a callback

export function clickOutside(node: HTMLElement, callback: () => void) {
  const handleClick = (event: MouseEvent) => {
    // Check if click is outside the node
    if (node && !node.contains(event.target as Node) && event.target instanceof Node) {
      callback();
    }
  };

  // Use setTimeout to ensure the listener is added after the current event loop
  // This prevents the initial click that opened the panel from immediately closing it
  const timeoutId = setTimeout(() => {
    document.addEventListener('click', handleClick, true);
  }, 10);

  return {
    destroy() {
      clearTimeout(timeoutId);
      document.removeEventListener('click', handleClick, true);
    }
  };
}
