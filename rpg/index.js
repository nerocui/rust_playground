import init, { wasm_main } from "./pkg/rpg.js";

async function run() {
  await init();
  wasm_main();
}

run();