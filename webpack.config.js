const path = require('path')

module.exports = {
    entry: path.join(__dirname, "react", "./image_uploader.jsx"),
    output: {
        path: path.resolve(__dirname, "./build"),
        filename: 'image_uploader.js'
    },
    module: {
        rules: [
            {
                test: /\.js|\.jsx$/,
                exclude: /node_modules/,
                use: {
                    loader: "babel-loader",
                    options: {
                        presets: ['@babel/preset-env', '@babel/preset-react']
                    }
                }
            }
        ]
    },
    resolve: {
        extensions: ['*', '.js', '.jsx']
    }
}