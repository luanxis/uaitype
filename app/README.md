# uaiType — app desktop (Linux)

Editor markdown **nativo** para Linux, feito em Rust com **GTK4 + libadwaita**.
Fase 1 do projeto: abrir/salvar `.md`, sidebar de workspace, preview renderizada
ao vivo na mesma janela (pulldown-cmark). WYSIWYG completo é a Fase 2.

## Funcionalidades (Fase 1)

- Abrir `.md` com `Ctrl+O`, `Ctrl+N` novo documento, `Ctrl+Q` sair
- Salvar com `Ctrl+S`, "salvar como" com `Ctrl+Shift+S`
- Editor com números de linha e syntax highlight markdown (esquemas monocromáticos/papel + blurple em `res/schemes/`)
- **Preview renderizada** na mesma janela (Ctrl+1 = fonte, Ctrl+2 = render): GFM — tabelas, task lists, strikethrough —, code fences com destaque, bloco de citação, mermaid
- **Workspace de pastas**: `Ctrl+Shift+O` escolhe a pasta; sidebar com **Arquivos** (árvore de pastas e `.md`) e **Índices** (lista plana de todos os `.md` do workspace)
- `Ctrl+B` ou botão na toolbar colapsa a sidebar; ela recolhe sozinha em janelas `< 700px` (breakpoints adw); toolbar se adapta abaixo de `460px`
- `+` no rodapé da sidebar cria `untitled[-N].md` dentro do workspace e o abre
- Indicador de "não salvo" (`●`) no título da janela com título vivo (`nome — uaiType`)
- Toasts para salvar/abrir/criar (`AdwToastOverlay`) e barra de status com contagem de palavras/caracteres
- Diálogos `AdwAboutDialog` (ícone folha em edição, MIT) e confirmação de descarte para fechar com alterações não salvas
- **Aparência** no menu ☰ → Sistema / Claro / Escuro (forçável em tempo real); editor abre no tema do sistema e alterna ao vivo
- Ícones em `res/icons/` (hicolor 16–512px + `-symbolic`) — launcher mostra o app pelo nome `io.github.luanxis.uaitype`
- Tema claro/escuro seguindo o sistema (libadwaita); contraste e esquema de cores quentes (papel envelhecido + blurple `#5865F2`)

## Pré-requisitos (no seu computador)

Pacotes de desenvolvimento do GTK4, libadwaita e SourceView5:

| Distro | Comando |
| --- | --- |
| Fedora | `sudo dnf install gtk4-devel libadwaita-devel gtksourceview5-devel cargo` |
| Debian / Ubuntu | `sudo apt install libgtk-4-dev libadwaita-1-dev libgtksourceview-5-dev cargo` |
| Arch | `sudo pacman -S gtk4 libadwaita gtksourceview5 rustup` |

Rust 1.75+ (testado compilando com o cargo disponível no ambiente de desenvolvimento).

## Rodar

```sh
cd app
cargo run
```

Build otimizado:

```sh
cargo build --release
./target/release/uaitype
```

## Instalando o ícone e o desktop file

O `application_id` é **`io.github.luanxis.uaitype`**; os arquivos de ícone e os
metadados (`.desktop`, `appdata.xml`) ficam em `res/icons/` e `data/`.
Para "enxergar" o app como um app de verdade (ícone no app grid, .md abrindo
com ele), instale no seu `~/.local/share`:

```sh
# ícones (hicolor: 16…512 em PNG + SVG escalável + variant symbolic)
for s in 16 32 48 64 128 256 512; do
  mkdir -p ~/.local/share/icons/hicolor/${s}x${s}/apps
  cp res/icons/hicolor/${s}x${s}/apps/io.github.luanxis.uaitype.png \
     ~/.local/share/icons/hicolor/${s}x${s}/apps/
done

mkdir -p ~/.local/share/icons/hicolor/scalable/apps
mkdir -p ~/.local/share/icons/hicolor/symbolic/apps
cp res/icons/io.github.luanxis.uaitype.svg \
   ~/.local/share/icons/hicolor/scalable/apps/
cp res/icons/io.github.luanxis.uaitype-symbolic.svg \
   ~/.local/share/icons/hicolor/symbolic/apps/

# cadê meu app, desktop
cp data/io.github.luanxis.uaitype.desktop ~/.local/share/applications/
update-desktop-database ~/.local/share/applications
```

Instale a raiz de uma vez no nível do sistema (requer sudo):

```sh
sudo cp -r res/icons/hicolor/* /usr/share/icons/hicolor/
sudo cp data/io.github.luanxis.uaitype.desktop /usr/share/applications/
sudo update-desktop-database
```

### Sistema de ícones

**Um único desenho**, escalável de 16px ao Flathub: "Folha em edição" —
página creme com canto dobrado, linhas de texto e caret blurple `#5865F2`
(inline na última linha), sobre squircle envelhecido `#171512`.

Geometria 100% vetorial (nada de dependência de fonte). O variant `-symbolic`
(monocromático, `currentColor`) serve menus e toolbar em apps GTK4; o favicon
do site também usa o mesmo desenho.

Para o Flathub: manifest Flatpak + `appdata.xml` + `icon.svg` escalável → PR no
`flathub/flatpak-text-editor`-style. O `appdata.xml` já declara `release 0.1.0`
e licença MIT; falta só a screenshot (gere uma com um compositor:
`gnome-screenshot -w -d 2`, salve em `docs/screenshot.png` e aponte o
`<screenshots>` do appdata para ela).

## Estrutura

```
app/
├── Cargo.toml                       # gtk4, libadwaita, sourceview5, pulldown-cmark
├── data/
│   ├── io.github.luanxis.uaitype.desktop      # .desktop Flathub/Linux
│   └── io.github.luanxis.uaitype.appdata.xml  # AppStream (NOME, licença, releases)
├── res/
│   ├── icons/                       # sistema de 2 ícones (ver tabela acima)
│   │   ├── io.github.luanxis.uaitype.svg              # "Caret mark" (marca/scalable)
│   │   ├── io.github.luanxis.uaitype-symbolic.svg     # mono, currentColor
│   │   └── hicolor/<n>x<n>/apps/io.github.luanxis.uaitype.png  # 16–512px, mesmo desenho
│   └── schemes/uaitype-dark.xml / uaitype-light.xml   # syntax highlight mono
└── src/
    └── main.rs                    # janela, editor, preview, ações de arquivo
```

## Limitações conhecidas

- **Requer WebKitGTK? Não.** É GTK nativo — precisa dos pacotes `*-devel` acima apenas para **compilar**; o app usa as libs do sistema em runtime.
- O preview cobre GFM: headings, bold/itálico, code inline/blocos, listas, links, tabelas, task lists e mermaid. Imagens externas, footnotes e WYSIWYG inline ficam para a Fase 2.
- Arquivos binários/imensas não são tratados; `.md` local apenas (caminhos fora do filesystem local, ex.: `gio` URIs, degradam para aviso no stderr).
- Fase 2 planejada: decorações inline no editor (marcadores de bold/heading escondidos fora da linha atual, estilo "Live Preview").

## Estrutura

```
app/
├── Cargo.toml     # gtk4, libadwaita, sourceview5, pulldown-cmark
└── src/
    └── main.rs    # janela, editor, preview, ações de arquivo (Ctrl+O/S/Shift+S)
```

## Licença

MIT — veja [`../LICENSE`](../LICENSE).
