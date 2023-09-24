<script setup lang="ts">
import { onMounted } from 'vue'
import * as wasm from "hello-wasm"

onMounted(() => {
  try {
    wasm.run_bevy_app()
  } catch (error: any) {
    if (!error.message.startsWith("Using exceptions for control flow,")) {
      throw error;
    }
  }

})

</script>

<template>
  <div class="wasm-container">
    <canvas id="bevy"></canvas>
  </div>
</template>

<style scoped>

.wasm-container {
  width: 100%;
  height: 100%;
  line-height: 0;
  margin: 0;
  padding: 0;
}

canvas {
  width: 100% !important;
  height: 100% !important;
  margin: 0;
  padding: 0;
  background: #2b2c2f;
}
</style>
