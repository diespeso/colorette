# Colorette
Color palette generation and extraction in the shape of a REST API (mostly experiments)

NOT READY YET

Built with Rocket.rs on the backend and React.js on the fronent

npm install babel-cli babel-preset-react-app

## to build javascript from react code (deprecated):
```bash
npx babel --watch react --out-dir build --presets react-app/prod
```

## use webpack to build the front end(only login.jsx right now)

webpack.config.js
```javascript
const path = require('path')

module.exports = {
    entry: path.join(__dirname, "react", "./login.jsx"),
    output: {
        path: path.resolve(__dirname, "./build"),
        filename: 'login.js'
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
```
package.json

```json
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1",
    "dev": "webpack serve",
    "build": "webpack"
  },
```

run as ```npm run build```

Theres a way to automatize building all files, might check out later
test

https://esausilva.com/2017/07/11/uncaught-referenceerror-regeneratorruntime-is-not-defined-two-solutions/
