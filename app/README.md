# uaiType — desktop app (Linux)

Native **markdown editor** for Linux, written in Rust with **GTK4 + libadwaita**.
Project phase 1: open/save `.md`, workspace sidebar, live GFM preview in the same window (pulldown-cmark).
[🇧🇷 Português](README.pt-BR.md)

## Features (Phase 1)

- Open `.md` with `Ctrl+O`, `Ctrl+N` new document, `Ctrl+Q` quit
- Save with `Ctrl+S`, "Save as" with `Ctrl+Shift+S`
- Editor with line numbers and markdown syntax highlighting (monochrome/editorial schemes + blurple in `res/schemes/`)
- **Preview in the same window** (Ctrl+1 = source, Ctrl+2 = render): GFM — tables, task lists, strikethrough —, highlighted code fences, blockquotes, mermaid diagrams
- **Workspace folders**: `Ctrl+Shift+O` picks the folder; sidebar with **Files** (folder tree of `.md` files) and **Indices** (flat list of every `.md` in the folder)
- `Ctrl+B` or the toolbar button collapses the sidebar; it collapses automatically on windows `< 700px` (Adw breakpoints); toolbar adapts below `460px`
- The `+` in the sidebar footer creates `untitled[-N].md` inside the workspace and opens it
- Unsaved indicator (`●`) in the window title with live title (`name — uaiType`)
- Toasts for save/open/create (`AdwToastOverlay`) and a status bar with word/char counts
- `AdwAboutDialog` (folha-em-edição icon, MIT) and discard confirmation when closing with unsaved changes
- **Appearance** in the ☰ menu → System / Light / Dark (live switchable); editor follows the system theme
- Icons in `res/icons/` (hicolor 16–512px + `-symbolic`) — the launcher shows the app as `io.github.luanxis.uaitype`
- Light/dark theme follows the system (libadwaita); warm scheme with blurple `#5865F2`

## Install (easiest)

Download the latest `.deb` or the tarball from the Releases page:

**Debian/Ubuntu (.deb)** — resolves dependencies automatically:

```sh
wget https://github.com/luanxis/uaitype/releases/latest/download/uaitype.deb
sudo apt install ./uaitype.deb
```

**Any Linux distro (tarball + script)** — installs to `~/.local` without sudo:

```sh
wget https://github.com/luanxis/uaitype/releases/latest/download/uaitype-linux-x86_64.tar.gz
tar xzf uaitype-linux-x86_64.tar.gz
cd uaitype-linux-x86_64 && ./install.sh
```

**System-wide (with sudo):** `sudo ./install.sh` (prefix `/usr/local`)
**Options:** `PREFIX=/opt ./install.sh` (custom prefix)

## Or build from source

### Prerequisites

| Distro | Command |
| --- | --- |
| Fedora | `sudo dnf install gtk4-devel libadwaita-devel gtksourceview5-devel cargo` |
| Debian / Ubuntu | `sudo apt install libgtk-4-dev libadwaita-1-dev libgtksourceview-5-dev cargo` |
| Arch | `sudo pacman -S gtk4 libadwaita gtksourceview5 rustup` |

## Run

```sh
cd app
cargo run
```

Optimized build:

```sh
cargo build --release
./target/release/uaitype
```

## Build the package yourself

```sh
cargo deb                       # target/debian/uaitype_0.1.0-1_amd64.deb
```

This packages the binary, the `.desktop`, the metainfo and the icons, resolving system dependencies automatically.

## Icon system

**A single design**, scalable from 16px to Flathub: "Folha em edição" — cream page with a folded corner, text lines and a blurple `#5865F2` caret (inline at the end of the last line) on a warm `#171512` squircle.

The `-symbolic` variant (monochrome, `currentColor`) is used by GTK4 menus and toolbars.

## Structure

```
app/
├── Cargo.toml                              # gtk4, libadwaita, sourceview5, pulldown-cmark (+ .deb metadata)
├── install.sh                              # installs the tarball (~/.local, or /usr/local with sudo)
├── data/
│   ├── io.github.luanxis.uaitype.desktop
│   └── io.github.luanxis.uaitype.metainfo.xml
├── res/
│   ├── icons/
│   │   ├── io.github.luanxis.uaitype.svg                 # master (squircle)
│   │   ├── io.github.luanxis.uaitype-symbolic.svg        # mono, currentColor
│   │   └── hicolor/<n>x<n>/apps/io.github.luanxis.uaitype.png
│   └── schemes/uaitype-dark.xml / uaitype-light.xml
└── src/
    └── main.rs                              # window, editor, preview, sidebar, actions
```

## Known limitations

- **No WebKitGTK needed.** It's native GTK — `*-devel` packages are required only to **build**; the app uses system libraries at runtime.
- The preview covers GFM: headings, bold/italic, inline/blocked code, lists, links, tables, task lists and mermaid. External images and footnotes land in Phase 2.
- Large binary files aren't handled; local `.md` only.
- Phase 2 planned: inline editor decorations (bold/heading markers hidden outside the current line).
- Works on any GTK4/libadwaita ≥ 1.5.

## License

MIT — see [`../LICENSE`](../LICENSE).
