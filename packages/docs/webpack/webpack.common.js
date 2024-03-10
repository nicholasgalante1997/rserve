const dotenv = require('dotenv');
const path = require('path');
const webpack = require('webpack');

dotenv.config();

/**
 * @type {import('webpack').Configuration}
 */
const config = {
    target: ['web', 'es2023'],
    module: {
        rules: [
          {
            test: /\.m?js/,
            type: 'javascript/auto',
            resolve: {
              fullySpecified: false
            }
          },
          {
            test: /\.css$/i,
            use: ['style-loader', 'css-loader']
          },
          {
            test: /\.json$/,
            loader: 'json-loader'
          },
          {
            test: /\.(ts|tsx|js|jsx)$/,
            exclude: /node_modules/,
            use: {
              loader: 'babel-loader'
            }
          },
          {
            test: /\.(png|webp|jpg|jpeg|avif|ico)/,
            type: 'asset/resource'
          }
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.jsx', '.js', '.json'],
        alias: {
            '@src': path.resolve(process.cwd(), 'src'),
            '@lib': path.resolve(process.cwd(), 'lib')
        },
        fallback: {
            path: false,
            process: false,
            fs: false
        }
    },
    plugins: [
        new webpack.ProvidePlugin({
          process: 'process/browser'
        }),
        new webpack.EnvironmentPlugin({ ...process.env })
    ]
};

module.exports = config;