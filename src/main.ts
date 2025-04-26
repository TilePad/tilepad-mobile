import { mount } from "svelte";

import App from "./App.svelte";

const app = mount(App, {
  target: document.getElementById("app")!,
});

// Add dev tools
if (import.meta.env.DEV) {
  import("eruda").then((eruda) => eruda.default.init());
}

export default app;
