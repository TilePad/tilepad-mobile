/* eslint-disable @typescript-eslint/no-explicit-any */

/**
 * Event emitting and subscribing
 */
export class EventEmitter<
  Events extends Record<string, (...args: any[]) => void>,
> {
  private events: { [K in keyof Events]?: Events[K][] };

  constructor() {
    this.events = {};
  }

  // Subscribe to an event
  on<K extends keyof Events>(event: K, callback: Events[K]) {
    if (!this.events[event]) {
      this.events[event] = [];
    }
    this.events[event]!.push(callback);
  }

  // Unsubscribe from an event
  off<K extends keyof Events>(event: K, callback: Events[K]) {
    if (!this.events[event]) return;
    this.events[event] = this.events[event]!.filter((cb) => cb !== callback);
  }

  // Emit an event
  emit<K extends keyof Events>(event: K, ...args: Parameters<Events[K]>) {
    if (!this.events[event]) return;
    this.events[event]!.forEach((callback) => callback(...args));
  }
}
