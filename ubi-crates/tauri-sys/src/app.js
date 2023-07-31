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

// tauri/tooling/api/src/app.ts
async function getVersion() {
  return invokeTauriCommand({
    __tauriModule: "App",
    message: {
      cmd: "getAppVersion"
    }
  });
}
async function getName() {
  return invokeTauriCommand({
    __tauriModule: "App",
    message: {
      cmd: "getAppName"
    }
  });
}
async function getTauriVersion() {
  return invokeTauriCommand({
    __tauriModule: "App",
    message: {
      cmd: "getTauriVersion"
    }
  });
}
async function show() {
  return invokeTauriCommand({
    __tauriModule: "App",
    message: {
      cmd: "show"
    }
  });
}
async function hide() {
  return invokeTauriCommand({
    __tauriModule: "App",
    message: {
      cmd: "hide"
    }
  });
}
export {
  getName,
  getTauriVersion,
  getVersion,
  hide,
  show
};
