# Debug log

Recorded fixes during Plan / Build / Debug for this workspace. Newest entries first.

---

## 2026-09-13 — Squirrel maker Authors is required

**Symptom:** `npm run make` failed on Windows: `Attempting to build package from 'azure-portfolio.nuspec'. Authors is required.`

**Root cause:** `@electron-forge/maker-squirrel` requires an `authors` field in package metadata / maker config.

**Code changed:**
- `package.json` — added `"author": "Azure"`.
- `forge.config.mjs` — `maker-squirrel` config `authors: 'Azure'`.

**How to read:** Squirrel `.nuspec` generation reads author metadata before producing `Setup.exe`.

**Verify:** `npm run make` — installer under `out/make/squirrel.windows/x64/`.

---

## 2026-09-13 — Electron app.asar bloated to 3.7 GB

**Symptom:** `npm run package` produced `out/Azure Portfolio-win32-x64` totaling ~4.1 GB; `resources/app.asar` was ~3.7 GB.

**Root cause:** Electron packager copied the whole repo into `app.asar` (`/target`, `/src`, `/static`, etc.). `.electronignore` alone did not exclude enough paths.

**Code changed:**
- `forge.config.mjs` — added `packagerConfig.ignore` whitelist-style exclusion for `src`, `target`, `dist`, `static`, `styles`, `node_modules`, etc.; Trunk `dist/` still ships via `extraResource`.
- `.electronignore` — expanded to match the same paths.

**How to read:** After `npm run package`, check `resources/app.asar` (expect under ~1 MB) and `resources/dist/` (~1 MB WASM bundle). Total `out/` is dominated by Electron + Chromium (~350–400 MB on Windows).

**Verify:** `npm run package` — `app.asar` ~0.4 MB, total `out/` ~369 MB.
