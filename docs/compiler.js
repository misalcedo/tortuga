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
    } else {
        play_button.classList.remove("hidden");
    }
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

    const code_block = playground_block.querySelector("code.language-tortuga");

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

function handle_hiding_boring(block) {
    const lines = Array.from(block.querySelectorAll('.boring'));

    // If no lines were hidden, return
    if (!lines.length) { return; }

    block.classList.add("hide-boring");

    const buttons = document.createElement('div');
    buttons.className = 'buttons';
    buttons.innerHTML = "<button class=\"fa fa-eye\" title=\"Show hidden lines\" aria-label=\"Show hidden lines\"></button>";

    // add expand button
    const pre_block = block.parentNode;
    pre_block.insertBefore(buttons, pre_block.firstChild);

    pre_block.querySelector('.buttons').addEventListener('click', function (e) {
        if (e.target.classList.contains('fa-eye')) {
            e.target.classList.remove('fa-eye');
            e.target.classList.add('fa-eye-slash');
            e.target.title = 'Hide lines';
            e.target.setAttribute('aria-label', e.target.title);

            block.classList.remove('hide-boring');
        } else if (e.target.classList.contains('fa-eye-slash')) {
            e.target.classList.remove('fa-eye-slash');
            e.target.classList.add('fa-eye');
            e.target.title = 'Show hidden lines';
            e.target.setAttribute('aria-label', e.target.title);

            block.classList.add('hide-boring');
        }
    });
}

function initialize_playground(pre_block) {
    // Add play button
    var buttons = pre_block.querySelector(".buttons");
    if (!buttons) {
        buttons = document.createElement('div');
        buttons.className = 'buttons';
        pre_block.insertBefore(buttons, pre_block.firstChild);
    }

    const runCodeButton = document.createElement('button');
    runCodeButton.className = 'fa fa-play play-button';
    runCodeButton.hidden = true;
    runCodeButton.title = 'Run this code';
    runCodeButton.setAttribute('aria-label', runCodeButton.title);

    buttons.insertBefore(runCodeButton, buttons.firstChild);
    runCodeButton.addEventListener('click', function (e) {
        run_tortuga_code(pre_block);
    });

    update_play_button(pre_block);

    const code_block = pre_block.querySelector("code");
    if (window.ace && code_block.classList.contains("editable")) {
        const undoChangesButton = document.createElement('button');
        undoChangesButton.className = 'fa fa-history reset-button';
        undoChangesButton.title = 'Undo changes';
        undoChangesButton.setAttribute('aria-label', undoChangesButton.title);

        buttons.insertBefore(undoChangesButton, buttons.firstChild);

        undoChangesButton.addEventListener('click', () => {
            let editor = window.ace.edit(code_block);
            editor.setValue(editor.originalCode);
            editor.clearSelection();
        });
    }
}

function initialize_code_blocks() {
    const code_blocks = Array.from(document.querySelectorAll("pre code.language-tortuga"));
    const playgrounds = code_blocks.map(c => c.parentNode);

    playgrounds.forEach(initialize_playground);
    code_blocks.forEach(handle_hiding_boring);
    playgrounds.forEach(run_editable);

    console.log(code_blocks);
    console.log(playgrounds);
}
