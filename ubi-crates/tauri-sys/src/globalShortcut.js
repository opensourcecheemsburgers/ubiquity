// tauri/tooling/api/src/tauri.ts
function uid() {
  return window.crypto.getRandomValues(new Uint32Array(1))[0];
}
function transformCallback(callback, once = false) {
  const identifier = uid();
  const prop = `_${identifier}`;
  Object.defineProperty(window, prop, {
    value: (result) => {
      if (once) {
        Reflect.deleteProperty(window, prop);
      }
      return callback?.(result);
    },
    writable: false,
    configurable: true
  });
  return identifier;
}
async function invoke(cmd, args = {}) {
  return new Promise((resolve, reject) => {
    const callback = transformCallback((e) => {
      resolve(e);
      Reflect.deleteProperty(window, `_${error}`);
    }, true);
    const error = transformCallback((e) => {
      reject(e);
      Reflect.deleteProperty(window, `_${callback}`);
    }, true);
    window.__TAURI_IPC__({
      cmd,
      callback,
      error,
      ...args
    });
  });
}

// tauri/tooling/api/src/helpers/tauri.ts
async function invokeTauriCommand(command) {
  return invoke("tauri", command);
}

// tauri/tooling/api/src/globalShortcut.ts
async function register(shortcut, handler) {
  return invokeTauriCommand({
    __tauriModule: "GlobalShortcut",
    message: {
      cmd: "register",
      shortcut,
      handler: transformCallback(handler)
    }
  });
}
async function registerAll(shortcuts, handler) {
  return invokeTauriCommand({
    __tauriModule: "GlobalShortcut",
    message: {
      cmd: "registerAll",
      shortcuts,
      handler: transformCallback(handler)
    }
  });
}
async function isRegistered(shortcut) {
  return invokeTauriCommand({
    __tauriModule: "GlobalShortcut",
    message: {
      cmd: "isRegistered",
      shortcut
    }
  });
}
async function unregister(shortcut) {
  return invokeTauriCommand({
    __tauriModule: "GlobalShortcut",
    message: {
      cmd: "unregister",
      shortcut
    }
  });
}
async function unregisterAll() {
  return invokeTauriCommand({
    __tauriModule: "GlobalShortcut",
    message: {
      cmd: "unregisterAll"
    }
  });
}
export {
  isRegistered,
  register,
  registerAll,
  unregister,
  unregisterAll
};
