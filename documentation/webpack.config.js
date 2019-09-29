const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin');
const dist = path.resolve(__dirname, "static", "js");
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
    mode: "development",
    devtool: "inline-source-map",
    entry: {
        "csp-wasm-editor": "./components/csp-wasm-editor/index.ts",
        "csp-wasm-pkg": "./components/csp-wasm-editor/pkg/index.js",
    },
    output: {
        globalObject: 'self',
        publicPath: '/js/',
        path: dist,
        filename: "[name].js",
    },
    module: {
        rules: [
            {
                test: /worker\.ts$/,
                use: {
                    loader: 'worker-loader',
                }
            },
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js', '.css', '.wasm'],
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.join(__dirname, 'components', 'csp-wasm-editor'),
            extraArgs: ' --target no-modules'
        }),
        new MonacoWebpackPlugin({
            languages: [],
            features: ['folding', 'contextmenu']
        }),
        new CopyPlugin([
            { from: path.join(__dirname, 'components', 'csp-wasm-editor', 'pkg'), to: dist}
        ])
    ],
    optimization: {
        splitChunks: {
            cacheGroups: {
                monaco: {
                    test: /node_modules\/monaco-editor\/esm\/vs\/editor/,
                    name: 'monaco',
                    chunks: 'all',
                    priority: 1
                },
                vendor: {
                    test: /node_modules\//,
                    name: 'vendors',
                    enforce: true,
                    chunks: 'all'
                }
            },
        },
    }
};
