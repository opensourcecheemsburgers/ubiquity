var __defProp = Object.defineProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};

// tauri/tooling/api/src/app.ts
var app_exports = {};
__export(app_exports, {
  getName: () => getName,
  getTauriVersion: () => getTauriVersion,
  getVersion: () => getVersion,
  hide: () => hide,
  show: () => show
});

// tauri/tooling/api/src/tauri.ts
var tauri_exports = {};
__export(tauri_exports, {
  convertFileSrc: () => convertFileSrc,
  invoke: () => invoke,
  transformCallback: () => transformCallback
});
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
  return new Promise((resolve2, reject) => {
    const callback = transformCallback((e) => {
      resolve2(e);
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
function convertFileSrc(filePath, protocol = "asset") {
  const path = encodeURIComponent(filePath);
  return navigator.userAgent.includes("Windows") ? `https://${protocol}.localhost/${path}` : `${protocol}://localhost/${path}`;
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

// tauri/tooling/api/src/cli.ts
var cli_exports = {};
__export(cli_exports, {
  getMatches: () => getMatches
});
async function getMatches() {
  return invokeTauriCommand({
    __tauriModule: "Cli",
    message: {
      cmd: "cliMatches"
    }
  });
}

// tauri/tooling/api/src/clipboard.ts
var clipboard_exports = {};
__export(clipboard_exports, {
  readText: () => readText,
  writeText: () => writeText
});
async function writeText(text) {
  return invokeTauriCommand({
    __tauriModule: "Clipboard",
    message: {
      cmd: "writeText",
      data: text
    }
  });
}
async function readText() {
  return invokeTauriCommand({
    __tauriModule: "Clipboard",
    message: {
      cmd: "readText",
      data: null
    }
  });
}

// tauri/tooling/api/src/dialog.ts
var dialog_exports = {};
__export(dialog_exports, {
  ask: () => ask,
  confirm: () => confirm,
  message: () => message,
  open: () => open,
  save: () => save
});
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

// tauri/tooling/api/src/event.ts
var event_exports = {};
__export(event_exports, {
  TauriEvent: () => TauriEvent,
  emit: () => emit2,
  listen: () => listen2,
  once: () => once2
});

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

// tauri/tooling/api/src/fs.ts
var fs_exports = {};
__export(fs_exports, {
  BaseDirectory: () => BaseDirectory,
  Dir: () => BaseDirectory,
  copyFile: () => copyFile,
  createDir: () => createDir,
  exists: () => exists,
  readBinaryFile: () => readBinaryFile,
  readDir: () => readDir,
  readTextFile: () => readTextFile,
  removeDir: () => removeDir,
  removeFile: () => removeFile,
  renameFile: () => renameFile,
  writeBinaryFile: () => writeBinaryFile,
  writeFile: () => writeTextFile,
  writeTextFile: () => writeTextFile
});
var BaseDirectory = /* @__PURE__ */ ((BaseDirectory2) => {
  BaseDirectory2[BaseDirectory2["Audio"] = 1] = "Audio";
  BaseDirectory2[BaseDirectory2["Cache"] = 2] = "Cache";
  BaseDirectory2[BaseDirectory2["Config"] = 3] = "Config";
  BaseDirectory2[BaseDirectory2["Data"] = 4] = "Data";
  BaseDirectory2[BaseDirectory2["LocalData"] = 5] = "LocalData";
  BaseDirectory2[BaseDirectory2["Desktop"] = 6] = "Desktop";
  BaseDirectory2[BaseDirectory2["Document"] = 7] = "Document";
  BaseDirectory2[BaseDirectory2["Download"] = 8] = "Download";
  BaseDirectory2[BaseDirectory2["Executable"] = 9] = "Executable";
  BaseDirectory2[BaseDirectory2["Font"] = 10] = "Font";
  BaseDirectory2[BaseDirectory2["Home"] = 11] = "Home";
  BaseDirectory2[BaseDirectory2["Picture"] = 12] = "Picture";
  BaseDirectory2[BaseDirectory2["Public"] = 13] = "Public";
  BaseDirectory2[BaseDirectory2["Runtime"] = 14] = "Runtime";
  BaseDirectory2[BaseDirectory2["Template"] = 15] = "Template";
  BaseDirectory2[BaseDirectory2["Video"] = 16] = "Video";
  BaseDirectory2[BaseDirectory2["Resource"] = 17] = "Resource";
  BaseDirectory2[BaseDirectory2["App"] = 18] = "App";
  BaseDirectory2[BaseDirectory2["Log"] = 19] = "Log";
  BaseDirectory2[BaseDirectory2["Temp"] = 20] = "Temp";
  BaseDirectory2[BaseDirectory2["AppConfig"] = 21] = "AppConfig";
  BaseDirectory2[BaseDirectory2["AppData"] = 22] = "AppData";
  BaseDirectory2[BaseDirectory2["AppLocalData"] = 23] = "AppLocalData";
  BaseDirectory2[BaseDirectory2["AppCache"] = 24] = "AppCache";
  BaseDirectory2[BaseDirectory2["AppLog"] = 25] = "AppLog";
  return BaseDirectory2;
})(BaseDirectory || {});
async function readTextFile(filePath, options = {}) {
  return invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "readTextFile",
      path: filePath,
      options
    }
  });
}
async function readBinaryFile(filePath, options = {}) {
  const arr = await invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "readFile",
      path: filePath,
      options
    }
  });
  return Uint8Array.from(arr);
}
async function writeTextFile(path, contents, options) {
  if (typeof options === "object") {
    Object.freeze(options);
  }
  if (typeof path === "object") {
    Object.freeze(path);
  }
  const file = { path: "", contents: "" };
  let fileOptions = options;
  if (typeof path === "string") {
    file.path = path;
  } else {
    file.path = path.path;
    file.contents = path.contents;
  }
  if (typeof contents === "string") {
    file.contents = contents ?? "";
  } else {
    fileOptions = contents;
  }
  return invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "writeFile",
      path: file.path,
      contents: Array.from(new TextEncoder().encode(file.contents)),
      options: fileOptions
    }
  });
}
async function writeBinaryFile(path, contents, options) {
  if (typeof options === "object") {
    Object.freeze(options);
  }
  if (typeof path === "object") {
    Object.freeze(path);
  }
  const file = { path: "", contents: [] };
  let fileOptions = options;
  if (typeof path === "string") {
    file.path = path;
  } else {
    file.path = path.path;
    file.contents = path.contents;
  }
  if (contents && "dir" in contents) {
    fileOptions = contents;
  } else if (typeof path === "string") {
    file.contents = contents ?? [];
  }
  return invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "writeFile",
      path: file.path,
      contents: Array.from(
        file.contents instanceof ArrayBuffer ? new Uint8Array(file.contents) : file.contents
      ),
      options: fileOptions
    }
  });
}
async function readDir(dir, options = {}) {
  return invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "readDir",
      path: dir,
      options
    }
  });
}
async function createDir(dir, options = {}) {
  return invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "createDir",
      path: dir,
      options
    }
  });
}
async function removeDir(dir, options = {}) {
  return invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "removeDir",
      path: dir,
      options
    }
  });
}
async function copyFile(source, destination, options = {}) {
  return invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "copyFile",
      source,
      destination,
      options
    }
  });
}
async function removeFile(file, options = {}) {
  return invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "removeFile",
      path: file,
      options
    }
  });
}
async function renameFile(oldPath, newPath, options = {}) {
  return invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "renameFile",
      oldPath,
      newPath,
      options
    }
  });
}
async function exists(path, options = {}) {
  return invokeTauriCommand({
    __tauriModule: "Fs",
    message: {
      cmd: "exists",
      path,
      options
    }
  });
}

// tauri/tooling/api/src/globalShortcut.ts
var globalShortcut_exports = {};
__export(globalShortcut_exports, {
  isRegistered: () => isRegistered,
  register: () => register,
  registerAll: () => registerAll,
  unregister: () => unregister,
  unregisterAll: () => unregisterAll
});
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

// tauri/tooling/api/src/http.ts
var http_exports = {};
__export(http_exports, {
  Body: () => Body,
  Client: () => Client,
  Response: () => Response,
  ResponseType: () => ResponseType,
  fetch: () => fetch,
  getClient: () => getClient
});
var ResponseType = /* @__PURE__ */ ((ResponseType2) => {
  ResponseType2[ResponseType2["JSON"] = 1] = "JSON";
  ResponseType2[ResponseType2["Text"] = 2] = "Text";
  ResponseType2[ResponseType2["Binary"] = 3] = "Binary";
  return ResponseType2;
})(ResponseType || {});
var Body = class {
  constructor(type2, payload) {
    this.type = type2;
    this.payload = payload;
  }
  static form(data) {
    const form = {};
    const append = (key, v) => {
      if (v !== null) {
        let r;
        if (typeof v === "string") {
          r = v;
        } else if (v instanceof Uint8Array || Array.isArray(v)) {
          r = Array.from(v);
        } else if (v instanceof File) {
          r = { file: v.name, mime: v.type, fileName: v.name };
        } else if (typeof v.file === "string") {
          r = { file: v.file, mime: v.mime, fileName: v.fileName };
        } else {
          r = { file: Array.from(v.file), mime: v.mime, fileName: v.fileName };
        }
        form[String(key)] = r;
      }
    };
    if (data instanceof FormData) {
      for (const [key, value] of data) {
        append(key, value);
      }
    } else {
      for (const [key, value] of Object.entries(data)) {
        append(key, value);
      }
    }
    return new Body("Form", form);
  }
  static json(data) {
    return new Body("Json", data);
  }
  static text(value) {
    return new Body("Text", value);
  }
  static bytes(bytes) {
    return new Body(
      "Bytes",
      Array.from(bytes instanceof ArrayBuffer ? new Uint8Array(bytes) : bytes)
    );
  }
};
var Response = class {
  constructor(response) {
    this.url = response.url;
    this.status = response.status;
    this.ok = this.status >= 200 && this.status < 300;
    this.headers = response.headers;
    this.rawHeaders = response.rawHeaders;
    this.data = response.data;
  }
};
var Client = class {
  constructor(id) {
    this.id = id;
  }
  async drop() {
    return invokeTauriCommand({
      __tauriModule: "Http",
      message: {
        cmd: "dropClient",
        client: this.id
      }
    });
  }
  async request(options) {
    const jsonResponse = !options.responseType || options.responseType === 1 /* JSON */;
    if (jsonResponse) {
      options.responseType = 2 /* Text */;
    }
    return invokeTauriCommand({
      __tauriModule: "Http",
      message: {
        cmd: "httpRequest",
        client: this.id,
        options
      }
    }).then((res) => {
      const response = new Response(res);
      if (jsonResponse) {
        try {
          response.data = JSON.parse(response.data);
        } catch (e) {
          if (response.ok && response.data === "") {
            response.data = {};
          } else if (response.ok) {
            throw Error(
              `Failed to parse response \`${response.data}\` as JSON: ${e};
              try setting the \`responseType\` option to \`ResponseType.Text\` or \`ResponseType.Binary\` if the API does not return a JSON response.`
            );
          }
        }
        return response;
      }
      return response;
    });
  }
  async get(url, options) {
    return this.request({
      method: "GET",
      url,
      ...options
    });
  }
  async post(url, body, options) {
    return this.request({
      method: "POST",
      url,
      body,
      ...options
    });
  }
  async put(url, body, options) {
    return this.request({
      method: "PUT",
      url,
      body,
      ...options
    });
  }
  async patch(url, options) {
    return this.request({
      method: "PATCH",
      url,
      ...options
    });
  }
  async delete(url, options) {
    return this.request({
      method: "DELETE",
      url,
      ...options
    });
  }
};
async function getClient(options) {
  return invokeTauriCommand({
    __tauriModule: "Http",
    message: {
      cmd: "createClient",
      options
    }
  }).then((id) => new Client(id));
}
var defaultClient = null;
async function fetch(url, options) {
  if (defaultClient === null) {
    defaultClient = await getClient();
  }
  return defaultClient.request({
    url,
    method: options?.method ?? "GET",
    ...options
  });
}

// tauri/tooling/api/src/notification.ts
var notification_exports = {};
__export(notification_exports, {
  isPermissionGranted: () => isPermissionGranted,
  requestPermission: () => requestPermission,
  sendNotification: () => sendNotification
});
async function isPermissionGranted() {
  if (window.Notification.permission !== "default") {
    return Promise.resolve(window.Notification.permission === "granted");
  }
  return invokeTauriCommand({
    __tauriModule: "Notification",
    message: {
      cmd: "isNotificationPermissionGranted"
    }
  });
}
async function requestPermission() {
  return window.Notification.requestPermission();
}
function sendNotification(options) {
  if (typeof options === "string") {
    new window.Notification(options);
  } else {
    new window.Notification(options.title, options);
  }
}

// tauri/tooling/api/src/path.ts
var path_exports = {};
__export(path_exports, {
  BaseDirectory: () => BaseDirectory,
  appCacheDir: () => appCacheDir,
  appConfigDir: () => appConfigDir,
  appDataDir: () => appDataDir,
  appDir: () => appDir,
  appLocalDataDir: () => appLocalDataDir,
  appLogDir: () => appLogDir,
  audioDir: () => audioDir,
  basename: () => basename,
  cacheDir: () => cacheDir,
  configDir: () => configDir,
  dataDir: () => dataDir,
  delimiter: () => delimiter,
  desktopDir: () => desktopDir,
  dirname: () => dirname,
  documentDir: () => documentDir,
  downloadDir: () => downloadDir,
  executableDir: () => executableDir,
  extname: () => extname,
  fontDir: () => fontDir,
  homeDir: () => homeDir,
  isAbsolute: () => isAbsolute,
  join: () => join,
  localDataDir: () => localDataDir,
  logDir: () => logDir,
  normalize: () => normalize,
  pictureDir: () => pictureDir,
  publicDir: () => publicDir,
  resolve: () => resolve,
  resolveResource: () => resolveResource,
  resourceDir: () => resourceDir,
  runtimeDir: () => runtimeDir,
  sep: () => sep,
  templateDir: () => templateDir,
  videoDir: () => videoDir
});

// tauri/tooling/api/src/helpers/os-check.ts
function isWindows() {
  return navigator.appVersion.includes("Win");
}

// tauri/tooling/api/src/path.ts
async function appDir() {
  return appConfigDir();
}
async function appConfigDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 21 /* AppConfig */
    }
  });
}
async function appDataDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 22 /* AppData */
    }
  });
}
async function appLocalDataDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 23 /* AppLocalData */
    }
  });
}
async function appCacheDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 24 /* AppCache */
    }
  });
}
async function audioDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 1 /* Audio */
    }
  });
}
async function cacheDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 2 /* Cache */
    }
  });
}
async function configDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 3 /* Config */
    }
  });
}
async function dataDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 4 /* Data */
    }
  });
}
async function desktopDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 6 /* Desktop */
    }
  });
}
async function documentDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 7 /* Document */
    }
  });
}
async function downloadDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 8 /* Download */
    }
  });
}
async function executableDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 9 /* Executable */
    }
  });
}
async function fontDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 10 /* Font */
    }
  });
}
async function homeDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 11 /* Home */
    }
  });
}
async function localDataDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 5 /* LocalData */
    }
  });
}
async function pictureDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 12 /* Picture */
    }
  });
}
async function publicDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 13 /* Public */
    }
  });
}
async function resourceDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 17 /* Resource */
    }
  });
}
async function resolveResource(resourcePath) {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: resourcePath,
      directory: 17 /* Resource */
    }
  });
}
async function runtimeDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 14 /* Runtime */
    }
  });
}
async function templateDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 15 /* Template */
    }
  });
}
async function videoDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 16 /* Video */
    }
  });
}
async function logDir() {
  return appLogDir();
}
async function appLogDir() {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolvePath",
      path: "",
      directory: 25 /* AppLog */
    }
  });
}
var sep = isWindows() ? "\\" : "/";
var delimiter = isWindows() ? ";" : ":";
async function resolve(...paths) {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "resolve",
      paths
    }
  });
}
async function normalize(path) {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "normalize",
      path
    }
  });
}
async function join(...paths) {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "join",
      paths
    }
  });
}
async function dirname(path) {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "dirname",
      path
    }
  });
}
async function extname(path) {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "extname",
      path
    }
  });
}
async function basename(path, ext) {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "basename",
      path,
      ext
    }
  });
}
async function isAbsolute(path) {
  return invokeTauriCommand({
    __tauriModule: "Path",
    message: {
      cmd: "isAbsolute",
      path
    }
  });
}

// tauri/tooling/api/src/process.ts
var process_exports = {};
__export(process_exports, {
  exit: () => exit,
  relaunch: () => relaunch
});
async function exit(exitCode = 0) {
  return invokeTauriCommand({
    __tauriModule: "Process",
    message: {
      cmd: "exit",
      exitCode
    }
  });
}
async function relaunch() {
  return invokeTauriCommand({
    __tauriModule: "Process",
    message: {
      cmd: "relaunch"
    }
  });
}

// tauri/tooling/api/src/shell.ts
var shell_exports = {};
__export(shell_exports, {
  Child: () => Child,
  Command: () => Command,
  EventEmitter: () => EventEmitter,
  open: () => open2
});
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
    return new Promise((resolve2, reject) => {
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
        resolve2({
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
async function open2(path, openWith) {
  return invokeTauriCommand({
    __tauriModule: "Shell",
    message: {
      cmd: "open",
      path,
      with: openWith
    }
  });
}

// tauri/tooling/api/src/updater.ts
var updater_exports = {};
__export(updater_exports, {
  checkUpdate: () => checkUpdate,
  installUpdate: () => installUpdate,
  onUpdaterEvent: () => onUpdaterEvent
});
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
  return new Promise((resolve2, reject) => {
    function onStatusChange(statusResult) {
      if (statusResult.error) {
        cleanListener();
        return reject(statusResult.error);
      }
      if (statusResult.status === "DONE") {
        cleanListener();
        return resolve2();
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
  return new Promise((resolve2, reject) => {
    function onUpdateAvailable(manifest) {
      cleanListener();
      return resolve2({
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
        return resolve2({
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

// tauri/tooling/api/src/window.ts
var window_exports = {};
__export(window_exports, {
  CloseRequestedEvent: () => CloseRequestedEvent,
  LogicalPosition: () => LogicalPosition,
  LogicalSize: () => LogicalSize,
  PhysicalPosition: () => PhysicalPosition,
  PhysicalSize: () => PhysicalSize,
  UserAttentionType: () => UserAttentionType,
  WebviewWindow: () => WebviewWindow,
  WebviewWindowHandle: () => WebviewWindowHandle,
  WindowManager: () => WindowManager,
  appWindow: () => appWindow,
  availableMonitors: () => availableMonitors,
  currentMonitor: () => currentMonitor,
  getAll: () => getAll,
  getCurrent: () => getCurrent,
  primaryMonitor: () => primaryMonitor
});
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

// tauri/tooling/api/src/os.ts
var os_exports = {};
__export(os_exports, {
  EOL: () => EOL,
  arch: () => arch,
  platform: () => platform,
  tempdir: () => tempdir,
  type: () => type,
  version: () => version
});
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

// tauri/tooling/api/src/index.ts
var invoke2 = invoke;
export {
  app_exports as app,
  cli_exports as cli,
  clipboard_exports as clipboard,
  dialog_exports as dialog,
  event_exports as event,
  fs_exports as fs,
  globalShortcut_exports as globalShortcut,
  http_exports as http,
  invoke2 as invoke,
  notification_exports as notification,
  os_exports as os,
  path_exports as path,
  process_exports as process,
  shell_exports as shell,
  tauri_exports as tauri,
  updater_exports as updater,
  window_exports as window
};
