import init from "./pkg/tortuga_site.js";
import { run }  from "./pkg/tortuga_site.js";

export function compileThenRun() {
    init("./pkg/tortuga_site_bg.wasm")
        .then(compileThenRunWasm)
        .catch(console.error);

    return false;
}

function compileThenRunWasm() {
    const input = document.getElementById("code") as HTMLTextAreaElement;
    const code = input.value;
    const result = run(code);
    const output = document.getElementById("output") as HTMLPreElement;

    output.innerText = result;
}