# Install

Install the requirements as described in the link below.
https://rustwasm.github.io/docs/book/game-of-life/setup.html 

# How to run

https://rustwasm.github.io/docs/book/game-of-life/hello-world.html#putting-it-into-a-web-page

1. Create wasm app
``` sh
cd app
npm init wasm-app www
```

2. Modify `index.js`

Edit `./www/index.js` like following.

``` javascript
import * as wasm from "wasm-rxrust";

wasm.greet();
```

3. Modify `package.json`

Add dependency like following.

``` json
  "dependencies": {
    "wasm-rxrust": "file:../pkg"
  },
  "devDependencies": {
    ... 
  }
```

4. Run

``` sh
wasm-pack build
npm install
npm run start
```
