const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const webpack = require('webpack');
const dist = path.resolve(__dirname, "static", "js");

module.exports = {
    mode: "development",
    entry: {
        "csp-wasm-editor": "./components/csp-wasm-editor/index.js"
    },
    output: {
        publicPath: '/js/',
        path: dist,
        filename: "[name].js"
    },
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
            {
                test: /\.js$/,
                exclude: /node_modules/,
                use: {
                    loader: "babel-loader"
                }
            }
        ]
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.join(__dirname, 'components', 'csp-wasm-editor'),
        })
    ]
};
