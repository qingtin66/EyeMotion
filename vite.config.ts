import { defineConfig } from 'vite';
import { resolve } from 'node:path';

export default defineConfig({
  root: 'ui',
  base: './',
  build: {
    outDir: '../dist',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(import.meta.dirname, 'ui/index.html'),
      },
    },
  },
  server: {
    port: 1420,
    strictPort: true,
  },
});
