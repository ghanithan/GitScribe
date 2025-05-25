# Rust‑Native Jamstack Builder – Design Document

*(version 0.1 – May 25 2025)*

---

## 1  Vision & Scope

Build and ship a **single‑binary, zero‑Node** static‑site generator (SSG) aimed at blogs and documentation.  All output is pre‑rendered HTML/CSS and can be deployed directly to any CDN, qualifying the tool as a **Jamstack builder** in its own right.  Initial release keeps the bundle ≤ 15 MB by deferring heavyweight features to later milestones.

---

## 2  Core Pipeline (MVP 0.1)

| Stage                | Tech / Crate                          | Notes                                                                  |
| -------------------- | ------------------------------------- | ---------------------------------------------------------------------- |
| File scan            | `ignore`                              | Recursively discover `*.md`, assets.                                   |
| Markdown → HTML      | `pulldown‑cmark`                      | Fast, CommonMark‑compatible.                                           |
| Template / Layouts   | `handlebars‑rust` *(Mustache syntax)* | Front‑matter values (YAML/TOML) + global site data merge into layouts. |
| CSS generation       | **Tailwind standalone CLI**           | Spawn executable shipped in `resources/`; `--minify` for prod.         |
| HTML post‑process    | DOM injection pass                    | Adds plugin snippets (head & tail), sets canonical URLs, etc.          |
| Write to `dist/`     | `std::fs`                             | Maintain deterministic hashes for cache‑busting later.                 |
| Local preview server | `axum`                                | Serves `dist/` on `127.0.0.1:<auto>`; optional live‑reload via SSE.    |

### CLI surface

```text
git-scribe build        # one‑shot render
git-scribe dev          # watch + live preview on http://127.0.0.1:4000
git-scribe plugin add <name>
git-scribe plugin rm  <name>
```

---

## 3  Project Layout

```
myblog/
├─ content/
│  ├─ 2025‑05‑25‑launch.md
│  └─ assets/hero.jpg
├─ templates/
│  ├─ base.hbs
│  └─ post.hbs
├─ plugins/
│  └─ boomerang/
│      ├─ boomerang.toml
│      └─ boomerang.js
├─ src/tailwind.css        # @tailwind directives
└─ dist/                   # (generated)
```

---

## 4  JavaScript Plugin System

Each plugin is a folder containing a **TOML manifest** and any asset files.

```toml
# plugins/boomerang/boomerang.toml
name    = "boomerang"
version = "0.1.0"

in_head = [
  '<script src="/assets/boomerang.js"></script>'
]

in_tail = [
  '<script>BOOMR.init({beacon_url: "/beacon"})</script>'
]

assets  = ["boomerang.js"]
```

* Build step copies `assets` → `dist/assets/` and injects `in_head` / `in_tail` snippets into every generated HTML file.
* Order is controlled by the list in `site.toml → plugins = [ … ]`.
* Duplicate‑tag detection prevents double injection during incremental builds.

---

## 5  Binary Footprint & Runtime Targets

| Item                   | Size (≈)    | Comment                                    |
| ---------------------- | ----------- | ------------------------------------------ |
| Rust SSG (stripped)    | 6 – 9 MB    | Cross‑compiled for x64, ARM.               |
| Tailwind CLI           | 3 – 5 MB    | One per‑OS binary; stored in `resources/`. |
| Total download         | **< 15 MB** | Meets lightweight goal.                    |
| RAM during `git-scribe dev` | 30 – 60 MB  | Depends on file‑watcher activity.          |

---

## 6  Roadmap After MVP

| Version | Add‑on                                                                        | Rationale                                 |
| ------- | ----------------------------------------------------------------------------- | ----------------------------------------- |
| **0.2** | Syntax highlighting via `syntect`; RSS feed generator                         | Completes common blogging features.       |
| **0.3** | Lossless image optimisation (`image`, `oxipng`); BlurHash placeholders        | Faster page‑load, better LCP.             |
| **0.4** | Shortcode/MDX‑like components using `comrak` extensions or WASM snippets      | Author interactive content without React. |
| **0.5** | Incremental builds + cache‑busting asset hashes                               | Large‑site performance.                   |
| **1.0** | Plug‑in API for external crates (dynamic library or WASM) + Theme marketplace | Community ecosystem; extensibility.       |

---

## 7  Desktop Wrapper (Optional)

*Wrap later using **Tauri 2.x** once CLI stabilises.*

* Tauri window hosts a `<iframe src="http://127.0.0.1:<port>" />` for live preview.
* Native dialogs (e.g., project picker) can be built with `tauri‑egui` if pure‑Rust widgets are desired.
* Keep the CLI as the single source‑of‑truth; the GUI simply shells out to `git-scribe` and streams logs.

---

## 8  Security & Offline Guarantees

* **No Node, no npm** – eliminates supply‑chain risk and large installs.
* All builds execute within user permissions; no network required unless a plugin specifies external CDN URLs.
* Optional sandbox: launch Tailwind & image optimisers inside a low‑privilege job (bwrap / macOS sandbox‑exec / Win32 JobObject).

---

## 9  Known Limitations

* No hybrid/ISR rendering; pages are fully static.
* No built‑in CMS or Git data layer (can be added via plugin fetching JSON/YAML at build‑time).
* Interactive islands require manual JS snippet plugins for now.

---

## 10  Next Steps (for the team)

1. Scaffold repo: `cargo new git-scribe` + workspaces for `core`, `cli`, `plugin-api`.
2. Integrate `pulldown‑cmark`, `handlebars`, `notify`, `axum`.
3. Bundle Tailwind CLI via `build.rs` download or `resources/` copy.
4. Implement plugin loader & injection pass.
5. Dog‑food: convert *this* document into a site using the MVP pipeline.

---

*Prepared by ChatGPT – compiled from engineering discussion May 25 2025.*
