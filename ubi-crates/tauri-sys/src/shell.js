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

// tauri/tooling/api/src/shell.ts
async function execute(onEvent, program, args = [], options) {
  if (typeof args === "object") {
    Object.freeze(args);
  }
  return invokeTauriCommand({
    __tauriModule: "Shell",
    message: {
      cmd: "execute",
      program,
      args,
      options,
      onEventFn: transformCallback(onEvent)
    }
  });
}
var EventEmitter = class {
  constructor() {
    this.eventListeners = /* @__PURE__ */ Object.create(null);
  }
  addListener(eventName, listener) {
    return this.on(eventName, listener);
  }
  removeListener(eventName, listener) {
    return this.off(eventName, listener);
  }
  on(eventName, listener) {
    if (eventName in this.eventListeners) {
      this.eventListeners[eventName].push(listener);
    } else {
      this.eventListeners[eventName] = [listener];
    }
    return this;
  }
  once(eventName, listener) {
    const wrapper = (...args) => {
      this.removeListener(eventName, wrapper);
      listener(...args);
    };
    return this.addListener(eventName, wrapper);
  }
  off(eventName, listener) {
    if (eventName in this.eventListeners) {
      this.eventListeners[eventName] = this.eventListeners[eventName].filter(
        (l) => l !== listener
      );
    }
    return this;
  }
  removeAllListeners(event) {
    if (event) {
      delete this.eventListeners[event];
    } else {
      this.eventListeners = /* @__PURE__ */ Object.create(null);
    }
    return this;
  }
  emit(eventName, ...args) {
    if (eventName in this.eventListeners) {
      const listeners = this.eventListeners[eventName];
      for (const listener of listeners)
        listener(...args);
      return true;
    }
    return false;
  }
  listenerCount(eventName) {
    if (eventName in this.eventListeners)
      return this.eventListeners[eventName].length;
    return 0;
  }
  prependListener(eventName, listener) {
    if (eventName in this.eventListeners) {
      this.eventListeners[eventName].unshift(listener);
    } else {
      this.eventListeners[eventName] = [listener];
    }
    return this;
  }
  prependOnceListener(eventName, listener) {
    const wrapper = (...args) => {
      this.removeListener(eventName, wrapper);
      listener(...args);
    };
    return this.prependListener(eventName, wrapper);
  }
};
var Child = class {
  constructor(pid) {
    this.pid = pid;
  }
  async write(data) {
    return invokeTauriCommand({
      __tauriModule: "Shell",
      message: {
        cmd: "stdinWrite",
        pid: this.pid,
        buffer: typeof data === "string" ? data : Array.from(data)
      }
    });
  }
  async kill() {
    return invokeTauriCommand({
      __tauriModule: "Shell",
      message: {
        cmd: "killChild",
        pid: this.pid
      }
    });
  }
};
var Command = class extends EventEmitter {
  constructor(program, args = [], options) {
    super();
    this.stdout = new EventEmitter();
    this.stderr = new EventEmitter();
    this.program = program;
    this.args = typeof args === "string" ? [args] : args;
    this.options = options ?? {};
  }
  static sidecar(program, args = [], options) {
    const instance = new Command(program, args, options);
    instance.options.sidecar = true;
    return instance;
  }
  async spawn() {
    return execute(
      (event) => {
        switch (event.event) {
          case "Error":
            this.emit("error", event.payload);
            break;
          case "Terminated":
            this.emit("close", event.payload);
            break;
          case "Stdout":
            this.stdout.emit("data", event.payload);
            break;
          case "Stderr":
            this.stderr.emit("data", event.payload);
            break;
        }
      },
      this.program,
      this.args,
      this.options
    ).then((pid) => new Child(pid));
  }
  async execute() {
    return new Promise((resolve, reject) => {
      this.on("error", reject);
      const stdout = [];
      const stderr = [];
      this.stdout.on("data", (line) => {
        stdout.push(line);
      });
      this.stderr.on("data", (line) => {
        stderr.push(line);
      });
      this.on("close", (payload) => {
        resolve({
          code: payload.code,
          signal: payload.signal,
          stdout: stdout.join("\n"),
          stderr: stderr.join("\n")
        });
      });
      this.spawn().catch(reject);
    });
  }
};
async function open(path, openWith) {
  return invokeTauriCommand({
    __tauriModule: "Shell",
    message: {
      cmd: "open",
      path,
      with: openWith
    }
  });
}
export {
  Child,
  Command,
  EventEmitter,
  open
};
