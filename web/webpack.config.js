const path = require('path');
module.exports = {
    entry: "./ts/index.ts",
    devtool: "source-map",
    output: {
        path: path.resolve(__dirname, "js"),
        filename: "index.js",
    },
    resolve: {
        extensions: ['.ts', '.js', '.wasm']
    },
    mode: "development",
    experiments: {
        asyncWebAssembly: true
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: "webassembly/async"
            },
            {
                test: /\.ts$/,
                loader: "ts-loader",
            },
        ],
    },
    devServer: {
        static: {
            directory: path.join(__dirname),
        },
        host: "0.0.0.0"
    }
};