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

// tauri/tooling/api/src/dialog.ts
async function open(options = {}) {
  if (typeof options === "object") {
    Object.freeze(options);
  }
  return invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "openDialog",
      options
    }
  });
}
async function save(options = {}) {
  if (typeof options === "object") {
    Object.freeze(options);
  }
  return invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "saveDialog",
      options
    }
  });
}
async function message(message2, options) {
  const opts = typeof options === "string" ? { title: options } : options;
  return invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "messageDialog",
      message: message2.toString(),
      title: opts?.title?.toString(),
      type: opts?.type
    }
  });
}
async function ask(message2, options) {
  const opts = typeof options === "string" ? { title: options } : options;
  return invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "askDialog",
      message: message2.toString(),
      title: opts?.title?.toString(),
      type: opts?.type
    }
  });
}
async function confirm(message2, options) {
  const opts = typeof options === "string" ? { title: options } : options;
  return invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "confirmDialog",
      message: message2.toString(),
      title: opts?.title?.toString(),
      type: opts?.type
    }
  });
}
export {
  ask,
  confirm,
  message,
  open,
  save
};
