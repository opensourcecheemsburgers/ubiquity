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

// tauri/tooling/api/src/http.ts
var ResponseType = /* @__PURE__ */ ((ResponseType2) => {
  ResponseType2[ResponseType2["JSON"] = 1] = "JSON";
  ResponseType2[ResponseType2["Text"] = 2] = "Text";
  ResponseType2[ResponseType2["Binary"] = 3] = "Binary";
  return ResponseType2;
})(ResponseType || {});
var Body = class {
  constructor(type, payload) {
    this.type = type;
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
export {
  Body,
  Client,
  Response,
  ResponseType,
  fetch,
  getClient
};
