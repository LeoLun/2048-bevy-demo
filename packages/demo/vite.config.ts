import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  base: '/2048-bevy-demo/',
  plugins: [
    vue(),
    wasm(),
    topLevelAwait()
  ],
})
