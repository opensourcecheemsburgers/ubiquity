// tauri/tooling/api/src/tauri.ts
function uid() {
  return window.crypto.getRandomValues(new Uint32Array(1))[0];
}
function transformCallback(callback, once2 = false) {
  const identifier = uid();
  const prop = `_${identifier}`;
  Object.defineProperty(window, prop, {
    value: (result) => {
      if (once2) {
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

// tauri/tooling/api/src/window.ts
var LogicalSize = class {
  constructor(width, height) {
    this.type = "Logical";
    this.width = width;
    this.height = height;
  }
};
var PhysicalSize = class {
  constructor(width, height) {
    this.type = "Physical";
    this.width = width;
    this.height = height;
  }
  toLogical(scaleFactor) {
    return new LogicalSize(this.width / scaleFactor, this.height / scaleFactor);
  }
};
var LogicalPosition = class {
  constructor(x, y) {
    this.type = "Logical";
    this.x = x;
    this.y = y;
  }
};
var PhysicalPosition = class {
  constructor(x, y) {
    this.type = "Physical";
    this.x = x;
    this.y = y;
  }
  toLogical(scaleFactor) {
    return new LogicalPosition(this.x / scaleFactor, this.y / scaleFactor);
  }
};
var UserAttentionType = /* @__PURE__ */ ((UserAttentionType2) => {
  UserAttentionType2[UserAttentionType2["Critical"] = 1] = "Critical";
  UserAttentionType2[UserAttentionType2["Informational"] = 2] = "Informational";
  return UserAttentionType2;
})(UserAttentionType || {});
function getCurrent() {
  return new WebviewWindow(window.__TAURI_METADATA__.__currentWindow.label, {
    skip: true
  });
}
function getAll() {
  return window.__TAURI_METADATA__.__windows.map(
    (w) => new WebviewWindow(w.label, {
      skip: true
    })
  );
}
var localTauriEvents = ["tauri://created", "tauri://error"];
var WebviewWindowHandle = class {
  constructor(label) {
    this.label = label;
    this.listeners = /* @__PURE__ */ Object.create(null);
  }
  async listen(event, handler) {
    if (this._handleTauriEvent(event, handler)) {
      return Promise.resolve(() => {
        const listeners = this.listeners[event];
        listeners.splice(listeners.indexOf(handler), 1);
      });
    }
    return listen(event, this.label, handler);
  }
  async once(event, handler) {
    if (this._handleTauriEvent(event, handler)) {
      return Promise.resolve(() => {
        const listeners = this.listeners[event];
        listeners.splice(listeners.indexOf(handler), 1);
      });
    }
    return once(event, this.label, handler);
  }
  async emit(event, payload) {
    if (localTauriEvents.includes(event)) {
      for (const handler of this.listeners[event] || []) {
        handler({ event, id: -1, windowLabel: this.label, payload });
      }
      return Promise.resolve();
    }
    return emit(event, this.label, payload);
  }
  _handleTauriEvent(event, handler) {
    if (localTauriEvents.includes(event)) {
      if (!(event in this.listeners)) {
        this.listeners[event] = [handler];
      } else {
        this.listeners[event].push(handler);
      }
      return true;
    }
    return false;
  }
};
var WindowManager = class extends WebviewWindowHandle {
  async scaleFactor() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "scaleFactor"
          }
        }
      }
    });
  }
  async innerPosition() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "innerPosition"
          }
        }
      }
    }).then(({ x, y }) => new PhysicalPosition(x, y));
  }
  async outerPosition() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "outerPosition"
          }
        }
      }
    }).then(({ x, y }) => new PhysicalPosition(x, y));
  }
  async innerSize() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "innerSize"
          }
        }
      }
    }).then(({ width, height }) => new PhysicalSize(width, height));
  }
  async outerSize() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "outerSize"
          }
        }
      }
    }).then(({ width, height }) => new PhysicalSize(width, height));
  }
  async isFullscreen() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "isFullscreen"
          }
        }
      }
    });
  }
  async isMaximized() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "isMaximized"
          }
        }
      }
    });
  }
  async isDecorated() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "isDecorated"
          }
        }
      }
    });
  }
  async isResizable() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "isResizable"
          }
        }
      }
    });
  }
  async isVisible() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "isVisible"
          }
        }
      }
    });
  }
  async theme() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "theme"
          }
        }
      }
    });
  }
  async center() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "center"
          }
        }
      }
    });
  }
  async requestUserAttention(requestType) {
    let requestType_ = null;
    if (requestType) {
      if (requestType === 1 /* Critical */) {
        requestType_ = { type: "Critical" };
      } else {
        requestType_ = { type: "Informational" };
      }
    }
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "requestUserAttention",
            payload: requestType_
          }
        }
      }
    });
  }
  async setResizable(resizable) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setResizable",
            payload: resizable
          }
        }
      }
    });
  }
  async setTitle(title) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setTitle",
            payload: title
          }
        }
      }
    });
  }
  async maximize() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "maximize"
          }
        }
      }
    });
  }
  async unmaximize() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "unmaximize"
          }
        }
      }
    });
  }
  async toggleMaximize() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "toggleMaximize"
          }
        }
      }
    });
  }
  async minimize() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "minimize"
          }
        }
      }
    });
  }
  async unminimize() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "unminimize"
          }
        }
      }
    });
  }
  async show() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "show"
          }
        }
      }
    });
  }
  async hide() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "hide"
          }
        }
      }
    });
  }
  async close() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "close"
          }
        }
      }
    });
  }
  async setDecorations(decorations) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setDecorations",
            payload: decorations
          }
        }
      }
    });
  }
  async setAlwaysOnTop(alwaysOnTop) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setAlwaysOnTop",
            payload: alwaysOnTop
          }
        }
      }
    });
  }
  async setSize(size) {
    if (!size || size.type !== "Logical" && size.type !== "Physical") {
      throw new Error(
        "the `size` argument must be either a LogicalSize or a PhysicalSize instance"
      );
    }
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setSize",
            payload: {
              type: size.type,
              data: {
                width: size.width,
                height: size.height
              }
            }
          }
        }
      }
    });
  }
  async setMinSize(size) {
    if (size && size.type !== "Logical" && size.type !== "Physical") {
      throw new Error(
        "the `size` argument must be either a LogicalSize or a PhysicalSize instance"
      );
    }
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setMinSize",
            payload: size ? {
              type: size.type,
              data: {
                width: size.width,
                height: size.height
              }
            } : null
          }
        }
      }
    });
  }
  async setMaxSize(size) {
    if (size && size.type !== "Logical" && size.type !== "Physical") {
      throw new Error(
        "the `size` argument must be either a LogicalSize or a PhysicalSize instance"
      );
    }
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setMaxSize",
            payload: size ? {
              type: size.type,
              data: {
                width: size.width,
                height: size.height
              }
            } : null
          }
        }
      }
    });
  }
  async setPosition(position) {
    if (!position || position.type !== "Logical" && position.type !== "Physical") {
      throw new Error(
        "the `position` argument must be either a LogicalPosition or a PhysicalPosition instance"
      );
    }
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setPosition",
            payload: {
              type: position.type,
              data: {
                x: position.x,
                y: position.y
              }
            }
          }
        }
      }
    });
  }
  async setFullscreen(fullscreen) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setFullscreen",
            payload: fullscreen
          }
        }
      }
    });
  }
  async setFocus() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setFocus"
          }
        }
      }
    });
  }
  async setIcon(icon) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setIcon",
            payload: {
              icon: typeof icon === "string" ? icon : Array.from(icon)
            }
          }
        }
      }
    });
  }
  async setSkipTaskbar(skip) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setSkipTaskbar",
            payload: skip
          }
        }
      }
    });
  }
  async setCursorGrab(grab) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setCursorGrab",
            payload: grab
          }
        }
      }
    });
  }
  async setCursorVisible(visible) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setCursorVisible",
            payload: visible
          }
        }
      }
    });
  }
  async setCursorIcon(icon) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setCursorIcon",
            payload: icon
          }
        }
      }
    });
  }
  async setCursorPosition(position) {
    if (!position || position.type !== "Logical" && position.type !== "Physical") {
      throw new Error(
        "the `position` argument must be either a LogicalPosition or a PhysicalPosition instance"
      );
    }
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setCursorPosition",
            payload: {
              type: position.type,
              data: {
                x: position.x,
                y: position.y
              }
            }
          }
        }
      }
    });
  }
  async setIgnoreCursorEvents(ignore) {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "setIgnoreCursorEvents",
            payload: ignore
          }
        }
      }
    });
  }
  async startDragging() {
    return invokeTauriCommand({
      __tauriModule: "Window",
      message: {
        cmd: "manage",
        data: {
          label: this.label,
          cmd: {
            type: "startDragging"
          }
        }
      }
    });
  }
  async onResized(handler) {
    return this.listen("tauri://resize" /* WINDOW_RESIZED */, handler);
  }
  async onMoved(handler) {
    return this.listen("tauri://move" /* WINDOW_MOVED */, handler);
  }
  async onCloseRequested(handler) {
    return this.listen("tauri://close-requested" /* WINDOW_CLOSE_REQUESTED */, (event) => {
      const evt = new CloseRequestedEvent(event);
      void Promise.resolve(handler(evt)).then(() => {
        if (!evt.isPreventDefault()) {
          return this.close();
        }
      });
    });
  }
  async onFocusChanged(handler) {
    const unlistenFocus = await this.listen(
      "tauri://focus" /* WINDOW_FOCUS */,
      (event) => {
        handler({ ...event, payload: true });
      }
    );
    const unlistenBlur = await this.listen(
      "tauri://blur" /* WINDOW_BLUR */,
      (event) => {
        handler({ ...event, payload: false });
      }
    );
    return () => {
      unlistenFocus();
      unlistenBlur();
    };
  }
  async onScaleChanged(handler) {
    return this.listen(
      "tauri://scale-change" /* WINDOW_SCALE_FACTOR_CHANGED */,
      handler
    );
  }
  async onMenuClicked(handler) {
    return this.listen("tauri://menu" /* MENU */, handler);
  }
  async onFileDropEvent(handler) {
    const unlistenFileDrop = await this.listen(
      "tauri://file-drop" /* WINDOW_FILE_DROP */,
      (event) => {
        handler({ ...event, payload: { type: "drop", paths: event.payload } });
      }
    );
    const unlistenFileHover = await this.listen(
      "tauri://file-drop-hover" /* WINDOW_FILE_DROP_HOVER */,
      (event) => {
        handler({ ...event, payload: { type: "hover", paths: event.payload } });
      }
    );
    const unlistenCancel = await this.listen(
      "tauri://file-drop-cancelled" /* WINDOW_FILE_DROP_CANCELLED */,
      (event) => {
        handler({ ...event, payload: { type: "cancel" } });
      }
    );
    return () => {
      unlistenFileDrop();
      unlistenFileHover();
      unlistenCancel();
    };
  }
  async onThemeChanged(handler) {
    return this.listen("tauri://theme-changed" /* WINDOW_THEME_CHANGED */, handler);
  }
};
var CloseRequestedEvent = class {
  constructor(event) {
    this._preventDefault = false;
    this.event = event.event;
    this.windowLabel = event.windowLabel;
    this.id = event.id;
  }
  preventDefault() {
    this._preventDefault = true;
  }
  isPreventDefault() {
    return this._preventDefault;
  }
};
var WebviewWindow = class extends WindowManager {
  constructor(label, options = {}) {
    super(label);
    if (!options?.skip) {
      invokeTauriCommand({
        __tauriModule: "Window",
        message: {
          cmd: "createWebview",
          data: {
            options: {
              label,
              ...options
            }
          }
        }
      }).then(async () => this.emit("tauri://created")).catch(async (e) => this.emit("tauri://error", e));
    }
  }
  static getByLabel(label) {
    if (getAll().some((w) => w.label === label)) {
      return new WebviewWindow(label, { skip: true });
    }
    return null;
  }
};
var appWindow;
if ("__TAURI_METADATA__" in window) {
  appWindow = new WebviewWindow(
    window.__TAURI_METADATA__.__currentWindow.label,
    {
      skip: true
    }
  );
} else {
  console.warn(
    `Could not find "window.__TAURI_METADATA__". The "appWindow" value will reference the "main" window label.
Note that this is not an issue if running this frontend on a browser instead of a Tauri window.`
  );
  appWindow = new WebviewWindow("main", {
    skip: true
  });
}
function mapMonitor(m) {
  return m === null ? null : {
    name: m.name,
    scaleFactor: m.scaleFactor,
    position: new PhysicalPosition(m.position.x, m.position.y),
    size: new PhysicalSize(m.size.width, m.size.height)
  };
}
async function currentMonitor() {
  return invokeTauriCommand({
    __tauriModule: "Window",
    message: {
      cmd: "manage",
      data: {
        cmd: {
          type: "currentMonitor"
        }
      }
    }
  }).then(mapMonitor);
}
async function primaryMonitor() {
  return invokeTauriCommand({
    __tauriModule: "Window",
    message: {
      cmd: "manage",
      data: {
        cmd: {
          type: "primaryMonitor"
        }
      }
    }
  }).then(mapMonitor);
}
async function availableMonitors() {
  return invokeTauriCommand({
    __tauriModule: "Window",
    message: {
      cmd: "manage",
      data: {
        cmd: {
          type: "availableMonitors"
        }
      }
    }
  }).then((ms) => ms.map(mapMonitor));
}
export {
  CloseRequestedEvent,
  LogicalPosition,
  LogicalSize,
  PhysicalPosition,
  PhysicalSize,
  UserAttentionType,
  WebviewWindow,
  WebviewWindowHandle,
  WindowManager,
  appWindow,
  availableMonitors,
  currentMonitor,
  getAll,
  getCurrent,
  primaryMonitor
};
