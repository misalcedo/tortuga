"use strict";

import init from "./pkg/tortuga_site.js";
import { run }  from "./pkg/tortuga_site.js";
import { playground_text }  from "./book.js";

init("./pkg/tortuga_site_bg.wasm")
    .then(initialize_code_blocks)
    .catch(console.error)

function update_play_button(pre_block) {
    const play_button = pre_block.querySelector(".play-button");

    // skip if code is `no_run`
    if (pre_block.querySelector('code').classList.contains("no_run")) {
        play_button.classList.add("hidden");
        return;
    }

    play_button.classList.remove("hidden");
}

function run_tortuga_code(code_block) {
    var result_block = code_block.querySelector(".result");
    if (!result_block) {
        result_block = document.createElement('code');
        result_block.className = 'result hljs language-bash';

        code_block.append(result_block);
    }

    const code = playground_text(code_block);

    result_block.innerText = "Running...";

    try {
        const result = run(code);

        if (result.trim() === '') {
            result_block.innerText = "No output";
            result_block.classList.add("result-no-output");
        } else {
            result_block.innerText = result;
            result_block.classList.remove("result-no-output");
        }
    } catch (error) {
        result_block.innerText = "Playground Communication: " + error.message;
    }
}

function run_editable(playground_block) {
    if (!window.ace) {
        return;
    }

    const code_block = playground_block.querySelector("code");

    if (!code_block.classList.contains("editable")) {
        return;
    }

    const editor = window.ace.edit(code_block);

    editor.addEventListener("change", function (e) {
        update_play_button(playground_block);
    });

    editor.commands.addCommand({
        name: "run",
        bindKey: {
            win: "Ctrl-Enter",
            mac: "Ctrl-Enter"
        },
        exec: _editor => run_tortuga_code(playground_block)
    });
}

function initialize_code_blocks() {
    const code_blocks = Array.from(document.querySelectorAll("pre code.language-tortuga"));
    const playgrounds = code_blocks.map(c => c.parentNode);

    playgrounds.forEach(update_play_button);
    playgrounds.forEach(run_editable);

    console.log("Hello, world!");
}
