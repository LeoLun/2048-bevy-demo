const { defineConfig } = require('@vue/cli-service')
const ModuleFederationPlugin = require("webpack").container.ModuleFederationPlugin;

module.exports = defineConfig({
  publicPath: "https://leolun.github.io/2048-bevy-demo/",
  transpileDependencies: true,
  configureWebpack: {
    plugins: [
      new ModuleFederationPlugin({
        name: "bevy_demo",
        filename: "remoteEntry.js",
        exposes: {
          "./WasmContainer": "./src/components/WasmContainer.vue",
        },
        shared: {
          vue: {
            singleton: true
          },
        }
      }),
    ],
    experiments: {
      asyncWebAssembly: true
    }
  }
})