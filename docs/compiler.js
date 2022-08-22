"use strict";

import init from "./pkg/tortuga_site.js";
import { run }  from "./pkg/tortuga_site.js";

function compileThenRun(event) {
    init("./pkg/tortuga_site_bg.wasm")
        .then(compileThenRunWasm)
        .catch(console.error);

    event.preventDefault();
}

function compileThenRunWasm() {
    const input = document.getElementById("code");
    const code = input.value;
    const result = run(code);
    const output = document.getElementById("output");

    output.innerText = result;
}

console.log("Hello, world!");