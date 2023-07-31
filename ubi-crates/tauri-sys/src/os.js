// tauri/tooling/api/src/helpers/os-check.ts
function isWindows() {
  return navigator.appVersion.includes("Win");
}

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

// tauri/tooling/api/src/os.ts
var EOL = isWindows() ? "\r\n" : "\n";
async function platform() {
  return invokeTauriCommand({
    __tauriModule: "Os",
    message: {
      cmd: "platform"
    }
  });
}
async function version() {
  return invokeTauriCommand({
    __tauriModule: "Os",
    message: {
      cmd: "version"
    }
  });
}
async function type() {
  return invokeTauriCommand({
    __tauriModule: "Os",
    message: {
      cmd: "osType"
    }
  });
}
async function arch() {
  return invokeTauriCommand({
    __tauriModule: "Os",
    message: {
      cmd: "arch"
    }
  });
}
async function tempdir() {
  return invokeTauriCommand({
    __tauriModule: "Os",
    message: {
      cmd: "tempdir"
    }
  });
}
export {
  EOL,
  arch,
  platform,
  tempdir,
  type,
  version
};
