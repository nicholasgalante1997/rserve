const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');
const { merge } = require('webpack-merge');
const commonConfig = require('./webpack.common');

/**
 * @type {import('webpack').Configuration}
 */
const config = {
    mode: 'development',
    entry: path.resolve(process.cwd(), 'src', 'index.tsx'),
    devServer: {
        hot: true,
        port: 3000,
        open: true,
        static: [
          {
            directory: path.resolve(process.cwd(), 'styles')
          },
          {
            directory: path.resolve(process.cwd(), 'assets')
          },
          {
            directory: path.resolve(process.cwd(), 'node_modules', 'heller-2-lite', 'build', 'css')
          }
        ]
      },
      plugins: [new HtmlWebpackPlugin({ template: 'public/index.html' })]
};

module.exports = merge(commonConfig, config);