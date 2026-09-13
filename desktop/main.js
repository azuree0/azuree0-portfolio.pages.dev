const { app, BrowserWindow, shell } = require("electron");
const fs = require("fs");
const http = require("http");
const path = require("path");

/** @type {import("http").Server | null} */
let server = null;

/** @type {import("electron").BrowserWindow | null} */
let mainWindow = null;

function getDistPath() {
  if (app.isPackaged) {
    return path.join(process.resourcesPath, "dist");
  }
  return path.join(__dirname, "..", "dist");
}

function contentType(filePath) {
  switch (path.extname(filePath).toLowerCase()) {
    case ".html":
      return "text/html; charset=utf-8";
    case ".js":
      return "application/javascript; charset=utf-8";
    case ".css":
      return "text/css; charset=utf-8";
    case ".wasm":
      return "application/wasm";
    case ".png":
      return "image/png";
    case ".json":
      return "application/json";
    case ".ico":
      return "image/x-icon";
    case ".svg":
      return "image/svg+xml";
    default:
      return "application/octet-stream";
  }
}

function startStaticServer(root) {
  return new Promise((resolve, reject) => {
    const resolvedRoot = path.resolve(root);

    const srv = http.createServer((req, res) => {
      const urlPath = decodeURIComponent((req.url || "/").split("?")[0]);
      const relativePath = urlPath === "/" ? "index.html" : urlPath.replace(/^\//, "");
      const filePath = path.resolve(resolvedRoot, relativePath);

      if (!filePath.startsWith(resolvedRoot)) {
        res.writeHead(403);
        res.end("Forbidden");
        return;
      }

      fs.readFile(filePath, (err, data) => {
        if (err) {
          if (urlPath !== "/" && !path.extname(urlPath)) {
            fs.readFile(path.join(resolvedRoot, "index.html"), (indexErr, indexData) => {
              if (indexErr) {
                res.writeHead(404);
                res.end("Not found");
                return;
              }
              res.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });
              res.end(indexData);
            });
            return;
          }
          res.writeHead(404);
          res.end("Not found");
          return;
        }

        res.writeHead(200, { "Content-Type": contentType(filePath) });
        res.end(data);
      });
    });

    srv.listen(0, "127.0.0.1", () => {
      const address = srv.address();
      if (!address || typeof address === "string") {
        reject(new Error("Failed to bind static server"));
        return;
      }
      resolve({ server: srv, port: address.port });
    });
    srv.on("error", reject);
  });
}

function isLocalAppUrl(url) {
  try {
    const parsed = new URL(url);
    return (
      (parsed.hostname === "127.0.0.1" || parsed.hostname === "localhost") &&
      parsed.protocol === "http:"
    );
  } catch {
    return false;
  }
}

async function createWindow() {
  const distPath = getDistPath();
  const { server: srv, port } = await startStaticServer(distPath);
  server = srv;

  mainWindow = new BrowserWindow({
    width: 1280,
    height: 800,
    autoHideMenuBar: true,
    webPreferences: {
      preload: path.join(__dirname, "preload.js"),
      contextIsolation: true,
      nodeIntegration: false,
    },
  });

  mainWindow.webContents.setWindowOpenHandler(({ url }) => {
    if (url.startsWith("http://") || url.startsWith("https://")) {
      shell.openExternal(url);
    }
    return { action: "deny" };
  });

  mainWindow.webContents.on("will-navigate", (event, url) => {
    if (isLocalAppUrl(url)) {
      return;
    }
    if (url.startsWith("http://") || url.startsWith("https://")) {
      event.preventDefault();
      shell.openExternal(url);
    }
  });

  await mainWindow.loadURL(`http://127.0.0.1:${port}/index.html`);
}

function closeServer() {
  if (server) {
    server.close();
    server = null;
  }
}

app.whenReady().then(createWindow);

app.on("window-all-closed", () => {
  closeServer();
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

app.on("before-quit", () => {
  closeServer();
});
