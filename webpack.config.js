const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, 'dist');

module.exports = {
  entry: './bootstrap.js',
  output: {
    path: distPath,
    filename: "yewchat.js",
  },
  devServer: {
    static: {
      directory: distPath,
    },
    compress: true,
    port: 8000,
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [ { from: './static', to: distPath } ],
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  }
};