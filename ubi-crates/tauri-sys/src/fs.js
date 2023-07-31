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
export {
  BaseDirectory,
  BaseDirectory as Dir,
  copyFile,
  createDir,
  exists,
  readBinaryFile,
  readDir,
  readTextFile,
  removeDir,
  removeFile,
  renameFile,
  writeBinaryFile,
  writeTextFile as writeFile,
  writeTextFile
};
