import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import federation from "@originjs/vite-plugin-federation";

// https://vitejs.dev/config/
export default defineConfig({
  base: '/2048-bevy-demo/',
  plugins: [
    vue(),
    wasm(),
    topLevelAwait(),
    federation({
      name: 'remote-app',
      filename: 'remoteEntry.js',
      // Modules to expose
      exposes: {
          './WasmContainer': './src/components/WasmContainer.vue',
      },
      shared: ['vue']
    })
  ],
})
