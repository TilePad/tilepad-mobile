/// <reference types="svelte" />
/// <reference types="vite/client" />
/// <reference types="unplugin-icons/types/svelte" />

declare namespace svelteHTML {
  interface HTMLAttributes {
    onswipeup?: (gestureEvent: GestureCustomEvent) => void;
    onswipedown?: (gestureEvent: GestureCustomEvent) => void;
    onswipemove?: (gestureEvent: GestureCustomEvent) => void;
    onswipe?: (e: SwipeCustomEvent) => void;
  }
}
