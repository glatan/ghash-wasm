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
  module: {
    rules: [
      {
        test: /\.scss/,
        use: [
          'style-loader',
          'css-loader',
          'sass-loader',
        ],
      },
    ],
  },
  plugins: [
    new CopyPlugin([
      {from: './static/', to: distPath, ignore: ['*.scss']}
    ]),
    new WasmPlugin({
      crateDirectory: ".",
      extraArgs: "--no-typescript"
    })
  ],
};
