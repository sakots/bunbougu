import { defineConfig } from 'vite';

export default defineConfig({
  clearScreen: false,
  server: {
    host: process.env.TAURI_DEV_HOST,
    port: 1420,
    strictPort: true,
    watch: { ignored: ['**/src-tauri/**'] },
  },
});
