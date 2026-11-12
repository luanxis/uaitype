# uaiType — app desktop (Linux)

Editor markdown **nativo** para Linux, feito em Rust com **GTK4 + libadwaita**.
Fase 1 do projeto: abrir/salvar `.md`, sidebar de workspace, preview renderizada
ao vivo na mesma janela (pulldown-cmark).

## Funcionalidades (Fase 1)

- Abrir `.md` com `Ctrl+O`, `Ctrl+N` novo documento, `Ctrl+Q` sair
- Salvar com `Ctrl+S`, "salvar como" com `Ctrl+Shift+S`
- Editor com números de linha e syntax highlight markdown (esquemas monocromáticos/papel + blurple em `res/schemes/`)
- **Preview renderizada** na mesma janela (Ctrl+1 = fonte, Ctrl+2 = render): GFM — tabelas, task lists, strikethrough —, code fences com destaque, bloco de citação, mermaid
- **Workspace de pastas**: `Ctrl+Shift+O` escolhe a pasta; sidebar com **Arquivos** (árvore de pastas e `.md`) e **Índices** (lista plana de todos os `.md` da pasta)
- `Ctrl+B` ou botão na toolbar colapsa a sidebar; ela recolhe sozinha em janelas `< 700px` (breakpoints adw); toolbar se adapta abaixo de `460px`
- `+` no rodapé da sidebar cria `untitled[-N].md` dentro do workspace e o abre
- Indicador de "não salvo" (`●`) no título da janela com título vivo (`nome — uaiType`)
- Toasts para salvar/abrir/criar (`AdwToastOverlay`) e barra de status com contagem de palavras/caracteres
- Diálogos `AdwAboutDialog` (ícone folha em edição, MIT) e confirmação de descarte para fechar com alterações não salvas
- **Aparência** no menu ☰ → Sistema / Claro / Escuro (forçável em tempo real); editor abre no tema do sistema e alterna ao vivo
- Ícones em `res/icons/` (hicolor 16–512px + `-symbolic`) — launcher mostra o app pelo nome `io.github.luanxis.uaitype`
- Tema claro/escuro seguindo o sistema (libadwaita); esquema quente com blurple `#5865F2`

## Instalação (o mais fácil)

Baixe o último `.deb` ou o tarball da página de releases:

**Debian/Ubuntu (.deb)** — resolve as dependências automaticamente:

```sh
wget https://github.com/luanxis/uaitype/releases/latest/download/uaitype.deb
sudo apt install ./uaitype.deb
```

**Qualquer distro Linux (tarball + script)** — instala em `~/.local` sem sudo:

```sh
wget https://github.com/luanxis/uaitype/releases/latest/download/uaitype-linux-x86_64.tar.gz
tar xzf uaitype-linux-x86_64.tar.gz
cd uaitype-linux-x86_64 && ./install.sh
```

**Com sudo (sistema todo):** `sudo ./install.sh` (prefixo `/usr/local`)
**Opções:** `PREFIX=/opt ./install.sh` (prefixo customizado)

## Ou compile do código-fonte

### Pré-requisitos

| Distro | Comando |
| --- | --- |
| Fedora | `sudo dnf install gtk4-devel libadwaita-devel gtksourceview5-devel cargo` |
| Debian / Ubuntu | `sudo apt install libgtk-4-dev libadwaita-1-dev libgtksourceview-5-dev cargo` |
| Arch | `sudo pacman -S gtk4 libadwaita gtksourceview5 rustup` |

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

## Gerar o pacote você mesmo

```sh
cargo deb                       # target/debian/uaitype_0.1.0-1_amd64.deb
```

Isso empacota o binário, o `.desktop`, o `metainfo.xml` e os ícones, resolvendo as dependências de sistema automaticamente.

## Sistema de ícones

**Um único desenho**, escalável de 16px ao Flathub: "Folha em edição" — página creme com canto dobrado, linhas de texto e caret blurple `#5865F2` (inline na última linha) sobre squircle envelhecido `#171512`.

O variant `-symbolic` (monocromático, `currentColor`) é o usado por menus e toolbar em apps GTK4.

## Estrutura

```
app/
├── Cargo.toml                              # gtk4, libadwaita, sourceview5, pulldown-cmark (+ metadata .deb)
├── install.sh                              # instala o tarball (~/.local ou /usr/local com sudo)
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
    └── main.rs                              # janela, editor, preview, sidebar e ações
```

## Limitações conhecidas

- **Requer WebKitGTK? Não.** É GTK nativo — os pacotes `*-devel` são necessários só para **compilar**; o app usa as libs do sistema em runtime.
- O preview cobre GFM: headings, bold/itálico, code inline/blocos, listas, links, tabelas, task lists e mermaid. Imagens externas e footnotes ficam para a Fase 2.
- Arquivos binários/imensos não são tratados; `.md` local apenas.
- Fase 2 planejada: decorações inline no editor (marcadores de bold/heading escondidos fora da linha atual).
- Sem dependência de `pprint`/bwrap; funciona em qualquer GTK4/libadwaita ≥ 1.5.

## Licença

MIT — veja [`../LICENSE`](../LICENSE).
