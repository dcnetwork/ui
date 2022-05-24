const invoke = window.__TAURI__.invoke

export async function invokeHello(name) {
    return await invoke("get_started", {name: name});
}

