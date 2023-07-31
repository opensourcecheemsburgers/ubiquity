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

// tauri/tooling/api/src/helpers/tauri.ts
async function invokeTauriCommand(command) {
  return invoke("tauri", command);
}

// tauri/tooling/api/src/fs.ts
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
export {
  BaseDirectory,
  appCacheDir,
  appConfigDir,
  appDataDir,
  appDir,
  appLocalDataDir,
  appLogDir,
  audioDir,
  basename,
  cacheDir,
  configDir,
  dataDir,
  delimiter,
  desktopDir,
  dirname,
  documentDir,
  downloadDir,
  executableDir,
  extname,
  fontDir,
  homeDir,
  isAbsolute,
  join,
  localDataDir,
  logDir,
  normalize,
  pictureDir,
  publicDir,
  resolve,
  resolveResource,
  resourceDir,
  runtimeDir,
  sep,
  templateDir,
  videoDir
};
