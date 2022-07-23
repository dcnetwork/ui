const invoke = window.__TAURI__.invoke

export async function invoke_START(name) {
    return await invoke("get_addr", {name: name});
}

export async function invoke_GET_PUB(name) {
    return await invoke("get_pub", {name: name});
}

