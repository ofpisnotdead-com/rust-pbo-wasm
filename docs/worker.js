importScripts('./pkg/pbo_files.js');

const {read_pbo_entries} = wasm_bindgen;

async function run_in_worker() {
  await wasm_bindgen('./pkg/pbo_files_bg.wasm');
}

run_in_worker();

onmessage = async function(e) {
  let entries = read_pbo_entries(
    e.data.file,
  );
  postMessage(entries);
}
