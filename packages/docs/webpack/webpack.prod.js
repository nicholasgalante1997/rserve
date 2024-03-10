const path = require('path');
const { merge } = require('webpack-merge');
const commonConfig = require('./webpack.common');

/**
 * @type {import('webpack').Configuration}
 */
const config = {
    mode: 'production',
    entry: path.resolve(process.cwd(), 'src', 'index.tsx'),
    output: {
        clean: false,
        path: path.resolve(process.cwd(), 'dist'),
        filename: '[name].bundle.js',
        module: true,
        chunkFormat: 'module'
      },
      experiments: {
        outputModule: true
      }
};

module.exports = merge(commonConfig, config);