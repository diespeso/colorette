# Colorette
Color palette generation and extraction in the shape of a REST API (mostly experiments)

NOT READY YET

## BRANCH FOR PERFORMANCE TWEAKS
apparently there are some nasty nested loops on main that made the program 300% slower.

#### With the old algorithm(1280x892, k=5, i=5):
![](https://github.com/diespeso/colorette/blob/engine_performance/img/old_algorithm.png)

#### With the new algorihtm(1280x892, k=5, i=5):
![](https://github.com/diespeso/colorette/blob/engine_performance/img/new_algorithm.png)

The color palette for both is pretty much the same, 3 blues, 1 brown, 1 yellow for the sunflower test image.


### input image / rgb color output
![](https://github.com/diespeso/colorette/blob/main/img/kmeans_input_result.png)

### display of the palette generated
![](https://github.com/diespeso/colorette/blob/main/img/coolors_palette.png)
Just to be clear, i ran the program 3 times and kept the best output, the other 2 outputs only had 1 yellow, one brown and 3 differente blue hues

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
