// @ts-check

/**
 * 開発用 ビルド & サーバー
 */
const fastify = require("fastify").default;
const fileSystem = require("fs").promises;
const childProcess = require("child_process");

const programJsPath = "dist/programJs";
const programWasmPath = "dist/programWasm";

childProcess.exec(
  "wasm-pack build --target no-modules",
  (execException, wasmPackOutput, wasmPackOutputErr) => {
    console.log(wasmPackOutput, wasmPackOutputErr);
    fileSystem.readFile("./pkg/rust_web_api.js").then((program) => {
      fileSystem.writeFile(
        programJsPath,
        program +
          `
        wasm_bindgen("wasm")
        `
      );
    });
    fileSystem.copyFile("./pkg/rust_web_api_bg.wasm", programWasmPath);
    // ビルド終了
    const instance = fastify();
    instance.get("/", (request, reply) => {
      reply.type("text/html");
      reply.send(`<!DOCTYPE html>
    <html lang="ja">
    
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Document</title>
        <script type="module" src="program">
        </script>
    </head>
    
    <body>
      init
    </body>
    
    </html>`);
    });
    instance.get("/program", (request, reply) => {
      reply.type("text/javascript");
      fileSystem.readFile(programJsPath).then((file) => {
        reply.send(file);
      });
    });
    instance.get("/wasm", (request, reply) => {
      reply.type("application/wasm");
      fileSystem.readFile(programWasmPath).then((file) => {
        reply.send(file);
      });
    });

    instance.listen(8080);
    console.log("http://localhost:8080");
  }
);
