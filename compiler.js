// deno-fmt-ignore-file
// deno-lint-ignore-file
// This code was bundled using `deno bundle` and it's not recommended to edit it manually

const importMeta = {
    url: "file:///home/runner/work/tortuga/tortuga/docs/book/pkg/tortuga_playground.js",
    main: false
};
let wasm;
const cachedTextDecoder = new TextDecoder('utf-8', {
    ignoreBOM: true,
    fatal: true
});
cachedTextDecoder.decode();
let cachedUint8Memory0 = new Uint8Array();
function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}
function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
const heap = new Array(32).fill(undefined);
heap.push(undefined, null, true, false);
let heap_next = heap.length;
function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];
    heap[idx] = obj;
    return idx;
}
function getObject(idx) {
    return heap[idx];
}
function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}
function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
let WASM_VECTOR_LEN = 0;
const cachedTextEncoder = new TextEncoder('utf-8');
const encodeString = typeof cachedTextEncoder.encodeInto === 'function' ? function(arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
} : function(arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
};
function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }
    let len = arg.length;
    let ptr1 = malloc(len);
    const mem = getUint8Memory0();
    let offset = 0;
    for(; offset < len; offset++){
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr1 + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr1 = realloc(ptr1, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr1 + offset, ptr1 + len);
        const ret = encodeString(arg, view);
        offset += ret.written;
    }
    WASM_VECTOR_LEN = offset;
    return ptr1;
}
let cachedInt32Memory0 = new Int32Array();
function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}
function run(input) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.run(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        var r3 = getInt32Memory0()[retptr / 4 + 3];
        var ptr1 = r0;
        var len1 = r1;
        if (r3) {
            ptr1 = 0;
            len1 = 0;
            throw takeObject(r2);
        }
        return getStringFromWasm0(ptr1, len1);
    } finally{
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(ptr1, len1);
    }
}
async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                } else {
                    throw e;
                }
            }
        }
        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);
        if (instance instanceof WebAssembly.Instance) {
            return {
                instance,
                module
            };
        } else {
            return instance;
        }
    }
}
function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_693216e109162396 = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally{
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    return imports;
}
function initMemory(imports, maybe_memory) {}
function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = new Int32Array();
    cachedUint8Memory0 = new Uint8Array();
    return wasm;
}
async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('tortuga_playground_bg.wasm', importMeta.url);
    }
    const imports = getImports();
    if (typeof input === 'string' || typeof Request === 'function' && input instanceof Request || typeof URL === 'function' && input instanceof URL) {
        input = fetch(input);
    }
    initMemory(imports);
    const { instance , module  } = await load(await input, imports);
    return finalizeInit(instance, module);
}
"use strict";
init("./pkg/tortuga_playground_bg.wasm").then(initialize_code_blocks).catch(console.error);
function update_play_button(pre_block) {
    const play_button = pre_block.querySelector(".play-button");
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
    editor.addEventListener("change", function(e) {
        update_play_button(playground_block);
    });
    editor.commands.addCommand({
        name: "run",
        bindKey: {
            win: "Ctrl-Enter",
            mac: "Ctrl-Enter"
        },
        exec: (_editor)=>run_tortuga_code(playground_block)
    });
}
function handle_hiding_boring(block) {
    const lines = Array.from(block.querySelectorAll('.boring'));
    if (!lines.length) {
        return;
    }
    block.classList.add("hide-boring");
    const buttons = document.createElement('div');
    buttons.className = 'buttons';
    buttons.innerHTML = "<button class=\"fa fa-eye\" title=\"Show hidden lines\" aria-label=\"Show hidden lines\"></button>";
    const pre_block = block.parentNode;
    pre_block.insertBefore(buttons, pre_block.firstChild);
    pre_block.querySelector('.buttons').addEventListener('click', function(e) {
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
    runCodeButton.addEventListener('click', function(e) {
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
        undoChangesButton.addEventListener('click', ()=>{
            let editor = window.ace.edit(code_block);
            editor.setValue(editor.originalCode);
            editor.clearSelection();
        });
    }
}
function initialize_code_blocks() {
    const code_blocks = Array.from(document.querySelectorAll("pre code.language-tortuga"));
    const playgrounds = code_blocks.map((c)=>c.parentNode);
    playgrounds.forEach(initialize_playground);
    code_blocks.forEach(handle_hiding_boring);
    playgrounds.forEach(run_editable);
}
