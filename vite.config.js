import { defineConfig } from "vite";
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';
import Icons from "unplugin-icons/vite";
import { svelte } from '@sveltejs/vite-plugin-svelte'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;


const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);


// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    svelte(),
    Icons({
      compiler: "svelte",
    }),
  ],

  resolve: {
    alias: {
      $lib: resolve(__dirname, 'src/lib'),
    },
  },

  build: {
    outDir: "build",
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1430,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
