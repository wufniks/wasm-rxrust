# 설치

https://rustwasm.github.io/docs/book/game-of-life/setup.html 의 설명을 따라 필요한 툴들을 설치한다.

# 예제

https://rustwasm.github.io/docs/book/game-of-life/hello-world.html#putting-it-into-a-web-page

1. 아래의 명령어를 실행한다.
``` sh
cd app
npm init wasm-app www
```

2. index 파일 내용을 수정한다.

`./www/index.js` 파일을 아래와 같이 수정한다.

``` javascript
import * as wasm from "wasm-rxrust";

wasm.greet();
```

3. `package.json` 파일을 아래와 같이 수정한다.

4. 실행

``` sh
wasm-pack build
npm install
npm run start
```
