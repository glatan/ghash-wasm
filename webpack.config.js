const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const distPath = path.resolve(__dirname, "./dist/");

module.exports = {
    experiments: {
      asyncWebAssembly: true
    },
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
      static: {
        directory: distPath,
      },
      host: '0.0.0.0',
      port: 8080,
      hot: true
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
      new CopyPlugin({
        patterns: [
          {
            from: './static/index.html',
            to: distPath,
          },
          {
            from: './static/favicon.png',
            to: distPath,
          },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "./pkg/"),
        watchDirectories: [
          path.resolve(__dirname, "./src/")
        ],
        extraArgs: "--no-typescript"
      })
    ],
};
