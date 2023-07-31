// tauri/tooling/api/src/mocks.ts
function mockIPC(cb) {
  window.__TAURI_IPC__ = async ({
    cmd,
    callback,
    error,
    ...args
  }) => {
    try {
      window[`_${callback}`](await cb(cmd, args));
    } catch (err) {
      window[`_${error}`](err);
    }
  };
}
function mockWindows(current, ...additionalWindows) {
  window.__TAURI_METADATA__ = {
    __windows: [current, ...additionalWindows].map((label) => ({ label })),
    __currentWindow: { label: current }
  };
}
function clearMocks() {
  delete window.__TAURI_IPC__;
  delete window.__TAURI_METADATA__;
}
export {
  clearMocks,
  mockIPC,
  mockWindows
};
