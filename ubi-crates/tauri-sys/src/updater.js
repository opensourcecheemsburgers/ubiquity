// tauri/tooling/api/src/tauri.ts
function uid() {
  return window.crypto.getRandomValues(new Uint32Array(1))[0];
}
function transformCallback(callback, once3 = false) {
  const identifier = uid();
  const prop = `_${identifier}`;
  Object.defineProperty(window, prop, {
    value: (result) => {
      if (once3) {
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

// tauri/tooling/api/src/helpers/event.ts
async function _unlisten(event, eventId) {
  return invokeTauriCommand({
    __tauriModule: "Event",
    message: {
      cmd: "unlisten",
      event,
      eventId
    }
  });
}
async function emit(event, windowLabel, payload) {
  await invokeTauriCommand({
    __tauriModule: "Event",
    message: {
      cmd: "emit",
      event,
      windowLabel,
      payload
    }
  });
}
async function listen(event, windowLabel, handler) {
  return invokeTauriCommand({
    __tauriModule: "Event",
    message: {
      cmd: "listen",
      event,
      windowLabel,
      handler: transformCallback(handler)
    }
  }).then((eventId) => {
    return async () => _unlisten(event, eventId);
  });
}
async function once(event, windowLabel, handler) {
  return listen(event, windowLabel, (eventData) => {
    handler(eventData);
    _unlisten(event, eventData.id).catch(() => {
    });
  });
}

// tauri/tooling/api/src/event.ts
async function listen2(event, handler) {
  return listen(event, null, handler);
}
async function once2(event, handler) {
  return once(event, null, handler);
}
async function emit2(event, payload) {
  return emit(event, void 0, payload);
}

// tauri/tooling/api/src/updater.ts
async function onUpdaterEvent(handler) {
  return listen2("tauri://update-status" /* STATUS_UPDATE */, (data) => {
    handler(data?.payload);
  });
}
async function installUpdate() {
  let unlistenerFn;
  function cleanListener() {
    if (unlistenerFn) {
      unlistenerFn();
    }
    unlistenerFn = void 0;
  }
  return new Promise((resolve, reject) => {
    function onStatusChange(statusResult) {
      if (statusResult.error) {
        cleanListener();
        return reject(statusResult.error);
      }
      if (statusResult.status === "DONE") {
        cleanListener();
        return resolve();
      }
    }
    onUpdaterEvent(onStatusChange).then((fn) => {
      unlistenerFn = fn;
    }).catch((e) => {
      cleanListener();
      throw e;
    });
    emit2("tauri://update-install" /* INSTALL_UPDATE */).catch((e) => {
      cleanListener();
      throw e;
    });
  });
}
async function checkUpdate() {
  let unlistenerFn;
  function cleanListener() {
    if (unlistenerFn) {
      unlistenerFn();
    }
    unlistenerFn = void 0;
  }
  return new Promise((resolve, reject) => {
    function onUpdateAvailable(manifest) {
      cleanListener();
      return resolve({
        manifest,
        shouldUpdate: true
      });
    }
    function onStatusChange(statusResult) {
      if (statusResult.error) {
        cleanListener();
        return reject(statusResult.error);
      }
      if (statusResult.status === "UPTODATE") {
        cleanListener();
        return resolve({
          shouldUpdate: false
        });
      }
    }
    once2("tauri://update-available" /* UPDATE_AVAILABLE */, (data) => {
      onUpdateAvailable(data?.payload);
    }).catch((e) => {
      cleanListener();
      throw e;
    });
    onUpdaterEvent(onStatusChange).then((fn) => {
      unlistenerFn = fn;
    }).catch((e) => {
      cleanListener();
      throw e;
    });
    emit2("tauri://update" /* CHECK_UPDATE */).catch((e) => {
      cleanListener();
      throw e;
    });
  });
}
export {
  checkUpdate,
  installUpdate,
  onUpdaterEvent
};
