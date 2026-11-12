# uaiType

A free, open source **native markdown editor** for Linux.
[🇧🇷 Português](README.pt-BR.md)

This repository contains:

- **Landing page** — static site pre-rendered with `hono ssg` (Hono + JSX SSR at build time, folder `src/`), deployed to **GitHub Pages** by the `deploy.yml` GitHub Action
- **Native Linux app** — `app/`: a native markdown editor built with **GTK4 + libadwaita** in Rust (Phase 1: open/save `.md`, workspace sidebar, flat index of `.md` files, mermaid and GFM tables in preview)

![uaiType](app/docs/screenshot.png)

## Run the site (dev)

```sh
npm install
npm run preview      # build + preview at http://localhost:4173/uaitype/
```

Iterate on `src/` and re-run `npm run preview` (the site is static — no server in production). To check a specific response, use `hono request /`.

## Static build and preview

```sh
npm run build        # pre-renders the site into dist/ (hono ssg)
npm run preview      # build + preview at http://localhost:4173/uaitype/
```

Production uses `hono ssg` (Hono's official pre-render). The site is served at **GitHub Pages** at `https://<owner>.github.io/uaitype/` — the asset base path (`base_path`/`BASE_PATH`, see `src/base.ts`) is `/uaitype/`; local dev uses `/`. The workflow `.github/workflows/deploy.yml` builds and deploys on every push to `main` (requires `Settings → Pages → Source: GitHub Actions` on the repo).

## Building the native app

See [`app/README.md`](app/README.md) — you need the development packages for GTK4/libadwaita/SourceView5, then `cargo run`.

## License

**MIT** — see [`LICENSE`](LICENSE).

You may use, copy, modify, merge, publish, distribute, sublicense and/or sell copies of uaiType, including commercially; you must keep the copyright notice and this license. No copyleft, no CLA, no asterisks.

The **name** uaiType identifies the official product; forks are allowed but must use a different name.
