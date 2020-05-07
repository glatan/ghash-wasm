const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPlugin = require("@wasm-tool/wasm-pack-plugin");

const distPath = path.resolve(__dirname, "dist");

module.exports = {
  entry: {
    index: './static/index.js'
  },
  output: {
    path: distPath,
    filename: 'index.js',
    chunkFilename: "[name].js",
    webassemblyModuleFilename: "ghash.wasm"
  },
  devServer: {
    contentBase: distPath,
    port: 8080
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),
    new WasmPlugin({
      crateDirectory: ".",
      extraArgs: "--no-typescript"
    })
  ],
};
