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
var TauriEvent = /* @__PURE__ */ ((TauriEvent2) => {
  TauriEvent2["WINDOW_RESIZED"] = "tauri://resize";
  TauriEvent2["WINDOW_MOVED"] = "tauri://move";
  TauriEvent2["WINDOW_CLOSE_REQUESTED"] = "tauri://close-requested";
  TauriEvent2["WINDOW_CREATED"] = "tauri://window-created";
  TauriEvent2["WINDOW_DESTROYED"] = "tauri://destroyed";
  TauriEvent2["WINDOW_FOCUS"] = "tauri://focus";
  TauriEvent2["WINDOW_BLUR"] = "tauri://blur";
  TauriEvent2["WINDOW_SCALE_FACTOR_CHANGED"] = "tauri://scale-change";
  TauriEvent2["WINDOW_THEME_CHANGED"] = "tauri://theme-changed";
  TauriEvent2["WINDOW_FILE_DROP"] = "tauri://file-drop";
  TauriEvent2["WINDOW_FILE_DROP_HOVER"] = "tauri://file-drop-hover";
  TauriEvent2["WINDOW_FILE_DROP_CANCELLED"] = "tauri://file-drop-cancelled";
  TauriEvent2["MENU"] = "tauri://menu";
  TauriEvent2["CHECK_UPDATE"] = "tauri://update";
  TauriEvent2["UPDATE_AVAILABLE"] = "tauri://update-available";
  TauriEvent2["INSTALL_UPDATE"] = "tauri://update-install";
  TauriEvent2["STATUS_UPDATE"] = "tauri://update-status";
  TauriEvent2["DOWNLOAD_PROGRESS"] = "tauri://update-download-progress";
  return TauriEvent2;
})(TauriEvent || {});
async function listen2(event, handler) {
  return listen(event, null, handler);
}
async function once2(event, handler) {
  return once(event, null, handler);
}
async function emit2(event, payload) {
  return emit(event, void 0, payload);
}
export {
  TauriEvent,
  emit2 as emit,
  listen2 as listen,
  once2 as once
};
