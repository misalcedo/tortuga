import init from "./pkg/tortuga_site.js";
import { run }  from "./pkg/tortuga_site.js";

function compileThenRun(event) {
    init("./pkg/tortuga_site_bg.wasm")
        .then(compileThenRunWasm)
        .catch(console.error);

    event.preventDefault();
}

function compileThenRunWasm() {
    const input = document.getElementById("code") as HTMLTextAreaElement;
    const code = input.value;
    const result = run(code);
    const output = document.getElementById("output") as HTMLPreElement;

    output.innerText = result;
}

document.getElementById("form").addEventListener("submit", compileThenRun);