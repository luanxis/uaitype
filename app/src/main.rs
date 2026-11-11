//! uaiType — editor markdown nativo para Linux.
//!
//! Janela libadwaita com dois painéis: à esquerda o editor de texto
//! (GtkSourceView com destaque de sintaxe markdown) e à direita a pré-visual
//! renderizada (TextView com TextTags). Abrir/Salvar/Salvar como com
//! Ctrl+O / Ctrl+S / Ctrl+Shift+S.

use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{gdk, gio};
use libadwaita as adw;
use libadwaita::prelude::*;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use sourceview5 as sourceview;
use sourceview5::prelude::{BufferExt, ViewExt};
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

struct AppState {
    current_path: RefCell<Option<PathBuf>>,
    dirty: Cell<bool>,
    workspace: RefCell<Option<PathBuf>>,
}

const WELCOME: &str = "# Bem-vindo ao uaiType\n\nUm editor markdown **nativo** para Linux, escrito em Rust com GTK4 e libadwaita.\n\n## O que funciona\n\n- **Abrir** arquivos `.md` com `Ctrl+O`\n- **Salvar** com `Ctrl+S` e **Salvar como** com `Ctrl+Shift+S`\n- Pré-visual atualizada ao digitar\n- Tema claro/escuro seguindo o seu sistema
- `Ctrl+1` = editar (fonte) · `Ctrl+2` = visualizar (render)
- `Ctrl+Shift+O` = abrir pasta (workspace) · `Ctrl+B` = mostrar/esconder sidebar
- Fechar a janela com `Ctrl+Q` (ou `Ctrl+W`)\n\n> Digite ao lado e veja este painel virar um documento.\n\n```rust\nfn main() {\n    println!(\"solta a ideia\");\n}\n```\n\n- item de tarefa\n- outro item\n\n## Diagramas Mermaid\n\n```mermaid\nflowchart LR\n    A[Abrir] --> B[Editar]\n    B --> C[Salvar]\n    B --> D[Exportar]\n```\n\n## Tabelas\n\n| Tecla | Função |\n| --- | --- |\n| `Ctrl+O` | Abrir |\n| `Ctrl+S` | Salvar |\n| `Ctrl+Shift+S` | Salvar como |\n";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id("io.github.luanxis.uaitype")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &libadwaita::Application) {
    let state = Rc::new(AppState {
        current_path: RefCell::new(None),
        dirty: Cell::new(false),
        workspace: RefCell::new(None),
    });

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("uaiType — untitled.md")
        .default_width(1180)
        .default_height(760)
        .build();

    // ---- layout ----
    let toolbar = adw::ToolbarView::new();
    let header = adw::HeaderBar::new();

    let window_title = adw::WindowTitle::new("uaiType — untitled.md", "uaiType");
    header.set_title_widget(Some(&window_title));

    let sidebar_toggle_btn = gtk4::Button::from_icon_name("sidebar-show-symbolic");
    sidebar_toggle_btn.set_tooltip_text(Some("Mostrar/esconder sidebar (Ctrl+B)"));
    // ---- editor ----
    let editor_buffer = sourceview::Buffer::new(None);
    editor_buffer.set_highlight_syntax(true);
    if let Some(lang) = sourceview::LanguageManager::default().language("markdown") {
        editor_buffer.set_language(Some(&lang));
    }
    let editor = sourceview::View::with_buffer(&editor_buffer);
    editor.add_css_class("editor");
    editor.set_wrap_mode(gtk4::WrapMode::Word);
    editor.set_show_line_numbers(true);

    // esquema do editor segue o tema do sistema (e acompanha trocas ao vivo)
    let source_style = custom_scheme_manager();
    let system_style = adw::StyleManager::default();
    apply_source_scheme(&source_style, &system_style, &editor_buffer);
    {
        let editor_buffer = editor_buffer.clone();
        let source_style = source_style.clone();
        system_style.connect_dark_notify(move |manager| {
            apply_source_scheme(&source_style, manager, &editor_buffer);
        });
    }

    // status bar inferior, estilo GNOME Text Editor
    let status_label = gtk4::Label::new(Some("0 palavras · 0 caracteres · UTF-8 · Markdown"));
    status_label.add_css_class("dim-label");
    status_label.set_margin_top(4);
    status_label.set_margin_bottom(4);
    status_label.set_margin_start(10);
    status_label.set_margin_end(10);
    toolbar.add_bottom_bar(&status_label);

    // toasts (save/open feedback)
    let toast_overlay = adw::ToastOverlay::new();
    toast_overlay.set_child(Some(&toolbar));
    window.set_content(Some(&toast_overlay));

    let parts = Rc::new(AppParts {
        window: window.clone(),
        buffer: editor_buffer.clone(),
        state: state.clone(),
        window_title: window_title.clone(),
        status_label: status_label.clone(),
        toast_overlay: toast_overlay.clone(),
    });
    let open_btn = gtk4::Button::from_icon_name("document-open-symbolic");
    open_btn.set_tooltip_text(Some("Abrir (Ctrl+O)"));
    let save_btn = gtk4::Button::from_icon_name("document-save-symbolic");
    save_btn.set_tooltip_text(Some("Salvar (Ctrl+S)"));

    // menu primário (po-over estilo GNOME)
    let doc_section = gio::Menu::new();
    doc_section.append(Some("Novo documento"), Some("app.new"));
    doc_section.append(Some("Abrir…"), Some("app.open"));
    doc_section.append(Some("Abrir pasta (workspace)…"), Some("app.open-folder"));
    doc_section.append(Some("Salvar"), Some("app.save"));
    doc_section.append(Some("Salvar como…"), Some("app.save-as"));

    let view_section = gio::Menu::new();
    view_section.append(Some("Fonte (editar)"), Some("app.view-editor"));
    view_section.append(Some("Visualização (render)"), Some("app.view-preview"));

    let appearance_section = gio::Menu::new();
    appearance_section.append(Some("Sistema"), Some("app.appearance-system"));
    appearance_section.append(Some("Claro"), Some("app.appearance-light"));
    appearance_section.append(Some("Escuro"), Some("app.appearance-dark"));
    let appearance_menu = gio::Menu::new();
    appearance_menu.append_submenu(Some("Aparência"), &appearance_section);

    let help_section = gio::Menu::new();
    help_section.append(Some("Sobre o uaiType"), Some("app.about"));

    let menu = gio::Menu::new();
    menu.append_section(None, &doc_section);
    menu.append_section(None, &view_section);
    menu.append_section(None, &appearance_menu);
    menu.append_section(None, &help_section);

    let menu_btn = gtk4::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .tooltip_text("Menu")
        .menu_model(&menu)
        .build();

    header.pack_start(&sidebar_toggle_btn);
    header.pack_start(&open_btn);
    header.pack_start(&save_btn);
    header.pack_end(&menu_btn);
    toolbar.add_top_bar(&header);

    let editor_scrolled = gtk4::ScrolledWindow::builder()
        .child(&editor)
        .hexpand(true)
        .vexpand(true)
        .propagate_natural_width(true)
        .propagate_natural_height(true)
        .min_content_width(240)
        .build();

    // ===== sidebar (workspace) =====
    let sidebar = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    sidebar.set_width_request(280);
    sidebar.add_css_class("sidebar");

    let sidebar_stack = gtk4::Stack::new();
    let files_list = gtk4::ListBox::new();
    let indices_list = gtk4::ListBox::new();
    files_list.add_css_class("sidebar-list");
    indices_list.add_css_class("sidebar-list");
    sidebar_stack.add_titled(&files_list, Some("files"), "Arquivos");
    sidebar_stack.add_titled(&indices_list, Some("indices"), "Índices");

    let switcher = gtk4::StackSwitcher::new();
    switcher.set_stack(Some(&sidebar_stack));
    switcher.set_halign(gtk4::Align::Center);
    switcher.set_margin_top(8);
    switcher.set_margin_bottom(6);
    let sidebar_scrolled = gtk4::ScrolledWindow::builder()
        .child(&sidebar_stack)
        .hexpand(true)
        .vexpand(true)
        .build();
    sidebar.append(&switcher);
    sidebar.append(&sidebar_scrolled);

    // rodapé da sidebar: [+] [pasta atual] [✕]
    let sidebar_footer = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    sidebar_footer.set_margin_start(6);
    sidebar_footer.set_margin_end(6);
    sidebar_footer.set_margin_top(6);
    sidebar_footer.set_margin_bottom(6);
    sidebar_footer.add_css_class("sidebar-footer");

    let new_file_btn = gtk4::Button::from_icon_name("list-add-symbolic");
    new_file_btn.add_css_class("flat");
    new_file_btn.set_tooltip_text(Some("Novo arquivo (Ctrl+N)"));

    let folder_btn = gtk4::Button::new();
    folder_btn.add_css_class("flat");
    let folder_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    let folder_icon = gtk4::Image::from_icon_name("folder-symbolic");
    folder_icon.set_pixel_size(16);
    let folder_label = gtk4::Label::new(Some("Abrir pasta…"));
    folder_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    folder_label.set_hexpand(true);
    folder_label.set_xalign(0.5);
    folder_box.append(&folder_icon);
    folder_box.append(&folder_label);
    folder_btn.set_child(Some(&folder_box));
    folder_btn.set_tooltip_text(Some("Abrir pasta… (Ctrl+Shift+O)"));

    let hide_sidebar_btn = gtk4::Button::from_icon_name("window-close-symbolic");
    hide_sidebar_btn.add_css_class("flat");
    hide_sidebar_btn.set_tooltip_text(Some("Esconder sidebar (Ctrl+B)"));

    sidebar_footer.append(&new_file_btn);
    sidebar_footer.append(&folder_btn);
    sidebar_footer.append(&hide_sidebar_btn);
    sidebar.append(&sidebar_footer);

    // handlers da sidebar
    {
        let parts = parts.clone();
        let files_list = files_list.clone();
        let indices_list = indices_list.clone();
        let folder_label = folder_label.clone();
        new_file_btn.connect_clicked(move |_| {
            // cria um .md novo no workspace e o abre
            let root = parts.state.workspace.borrow().clone();
            let Some(root) = root else {
                show_toast(
                    &parts.toast_overlay,
                    "Abra uma pasta primeiro (Ctrl+Shift+O).",
                );
                return;
            };
            let target = unique_untitled(&root);
            match std::fs::File::create(&target) {
                Ok(_) => {
                    refresh_workspace(&files_list, &indices_list, &folder_label, &parts);
                    open_file_direct(
                        parts.window.clone(),
                        parts.buffer.clone(),
                        parts.state.clone(),
                        target.clone(),
                        parts.window_title.clone(),
                        parts.toast_overlay.clone(),
                    );
                    show_toast(
                        &parts.toast_overlay,
                        &format!("Criado: {}", target.file_name().unwrap().to_string_lossy()),
                    );
                }
                Err(err) => {
                    eprintln!("uaiType: falha ao criar {}: {err}", target.display());
                    show_toast(&parts.toast_overlay, "Não foi possível criar o arquivo.");
                }
            }
        });
    }
    {
        let parts = parts.clone();
        let files_list = files_list.clone();
        let indices_list = indices_list.clone();
        let folder_label = folder_label.clone();
        let window = window.clone();
        folder_btn.connect_clicked(move |_| {
            choose_workspace_folder(&window, &files_list, &indices_list, &folder_label, &parts);
        });
    }
    let split_view_slot: Rc<RefCell<Option<adw::NavigationSplitView>>> =
        Rc::new(RefCell::new(None));
    {
        let split_view_slot = split_view_slot.clone();
        hide_sidebar_btn.connect_clicked(move |_| {
            if let Some(view) = split_view_slot.borrow().as_ref() {
                view.set_collapsed(true);
            }
        });
    }

    // popula o estado inicial (vazio, sem vault)
    refresh_workspace(&files_list, &indices_list, &folder_label, &parts);

    // ---- preview (widgets) ----
    let preview_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    preview_box.add_css_class("preview");
    preview_box.set_margin_start(18);
    preview_box.set_margin_end(18);
    preview_box.set_margin_top(14);
    preview_box.set_margin_bottom(28);

    // coluna de leitura com largura máxima (o padrão do GNOME: ScrolledWindow > Viewport > AdwClamp > conteúdo)
    let preview_clamp = adw::Clamp::builder()
        .child(&preview_box)
        .maximum_size(720)
        .tightening_threshold(280)
        .build();
    let preview_viewport =
        gtk4::Viewport::new(None::<&gtk4::Adjustment>, None::<&gtk4::Adjustment>);
    preview_viewport.set_child(Some(&preview_clamp));
    let preview_scrolled = gtk4::ScrolledWindow::builder()
        .child(&preview_viewport)
        .hexpand(true)
        .vexpand(true)
        .propagate_natural_width(true)
        .propagate_natural_height(true)
        .build();

    // dois modos: fonte (editar) e visualização — alterna com Ctrl+1 / Ctrl+2
    let stack = gtk4::Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    stack.set_transition_duration(160);
    stack.add_named(&editor_scrolled, Some("editor"));
    stack.add_named(&preview_scrolled, Some("preview"));
    stack.set_visible_child_name("editor");
    // NavigationSplitView: a sidebar colapsa sozinha quando a janela fica estreita
    let sidebar_page = adw::NavigationPage::builder()
        .title("Arquivos")
        .child(&sidebar)
        .build();
    let content_page = adw::NavigationPage::builder()
        .title("uaiType")
        .child(&stack)
        .build();
    let split_view = adw::NavigationSplitView::builder()
        .sidebar(&sidebar_page)
        .content(&content_page)
        .min_sidebar_width(220.0)
        .max_sidebar_width(380.0)
        .sidebar_width_fraction(0.28)
        .build();
    *split_view_slot.borrow_mut() = Some(split_view.clone());
    toolbar.set_content(Some(&split_view));
    // status bar inferior, estilo GNOME Text Editor

    // ---- CSS ----
    let css = gtk4::CssProvider::new();
    css.load_from_data(
        "@media (prefers-color-scheme: dark) {\n\
           @define-color window_bg_color #171512;\n\
           @define-color view_bg_color #171512;\n\
           @define-color headerbar_bg_color #1B1815;\n\
           @define-color popover_bg_color #1F1C19;\n\
           @define-color card_bg_color #1F1C19;\n\
           @define-color accent_color #5865F2;\n\
           @define-color accent_fg_color #FFFFFF;\n\
         }\n\
         @media (prefers-color-scheme: light) {\n\
           @define-color window_bg_color #FAF8F5;\n\
           @define-color view_bg_color #FAF8F5;\n\
           @define-color headerbar_bg_color #FAF8F5;\n\
           @define-color popover_bg_color #FFFFFF;\n\
           @define-color card_bg_color #F2EFE9;\n\
           @define-color accent_color #4752C4;\n\
           @define-color accent_fg_color #FFFFFF;\n\
         }\n\
         .editor { font-family: monospace; font-size: 13px; }\n\
         .preview-p { color: @view_fg_color; }\n\
         .preview-h1 { font-size: 23px; font-weight: 700; }\n\
         .preview-h2 { font-size: 18px; font-weight: 700; margin-top: 6px; }\n\
         .preview-h3 { font-size: 15px; font-weight: 700; margin-top: 6px; }\n\
         .preview-code {\n\
           font-family: 'JetBrains Mono', monospace;\n\
           background: alpha(@view_fg_color, 0.09);\n\
           border: 1px solid alpha(@view_fg_color, 0.12);\n\
           border-radius: 8px;\n\
           padding: 12px 14px;\n\
         }\n\
         .preview-quote {\n\
           border-left: 3px solid rgba(88, 101, 242, 0.55);\n\
           padding-left: 12px;\n\
           font-style: italic;\n\
         }\n\
         .preview-picture { border: 1px solid alpha(@view_fg_color, 0.14); border-radius: 12px; background: alpha(@view_fg_color, 0.06); padding: 10px; }\n\
         .preview-cell { color: @view_fg_color; }\n\
         .preview-cell-head { font-weight: 600; color: @view_fg_color; border-bottom: 1px solid alpha(@view_fg_color, 0.18); padding-bottom: 4px; }",
    );
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &css,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // ---- ações (atalhos) ----
    let open_action = gio::SimpleAction::new("open", None);
    let save_action = gio::SimpleAction::new("save", None);
    let save_as_action = gio::SimpleAction::new("save-as", None);

    {
        let state = state.clone();
        let buffer = editor_buffer.clone();
        let window = window.clone();
        let window_title = window_title.clone();
        let toast_overlay = toast_overlay.clone();
        open_action.connect_activate(move |_, _| {
            open_markdown(
                window.clone(),
                buffer.clone(),
                state.clone(),
                window_title.clone(),
                toast_overlay.clone(),
            );
        });
    }
    {
        let state = state.clone();
        let buffer = editor_buffer.clone();
        let window = window.clone();
        let window_title = window_title.clone();
        let toast_overlay = toast_overlay.clone();
        save_action.connect_activate(move |_, _| {
            save_markdown(
                window.clone(),
                buffer.clone(),
                state.clone(),
                window_title.clone(),
                toast_overlay.clone(),
            );
        });
    }
    {
        let state = state.clone();
        let buffer = editor_buffer.clone();
        let window = window.clone();
        let window_title = window_title.clone();
        let toast_overlay = toast_overlay.clone();
        save_as_action.connect_activate(move |_, _| {
            save_markdown_as(
                window.clone(),
                buffer.clone(),
                state.clone(),
                window_title.clone(),
                toast_overlay.clone(),
            );
        });
    }

    let view_editor_action = gio::SimpleAction::new("view-editor", None);
    {
        let stack = stack.clone();
        view_editor_action.connect_activate(move |_, _| {
            stack.set_visible_child_name("editor");
        });
    }
    app.add_action(&view_editor_action);
    app.set_accels_for_action("app.view-editor", &["<Control>1"]);

    let view_preview_action = gio::SimpleAction::new("view-preview", None);
    {
        let stack = stack.clone();
        view_preview_action.connect_activate(move |_, _| {
            stack.set_visible_child_name("preview");
        });
    }
    app.add_action(&view_preview_action);
    app.set_accels_for_action("app.view-preview", &["<Control>2"]);

    let new_action = gio::SimpleAction::new("new", None);
    {
        let window = window.clone();
        let buffer = editor_buffer.clone();
        let state = state.clone();
        let window_title = window_title.clone();
        let status_label = status_label.clone();
        let toast_overlay = toast_overlay.clone();
        new_action.connect_activate(move |_, _| {
            new_document(
                window.clone(),
                buffer.clone(),
                state.clone(),
                window_title.clone(),
                status_label.clone(),
                toast_overlay.clone(),
            );
        });
    }
    app.add_action(&new_action);
    app.set_accels_for_action("app.new", &["<Control>n"]);

    let about_action = gio::SimpleAction::new("about", None);
    {
        let window = window.clone();
        about_action.connect_activate(move |_, _| {
            show_about(&window);
        });
    }
    app.add_action(&about_action);

    let open_folder_action = gio::SimpleAction::new("open-folder", None);
    {
        let window = window.clone();
        let files_list = files_list.clone();
        let indices_list = indices_list.clone();
        let folder_label = folder_label.clone();
        let parts = parts.clone();
        open_folder_action.connect_activate(move |_, _| {
            choose_workspace_folder(&window, &files_list, &indices_list, &folder_label, &parts);
        });
    }
    app.add_action(&open_folder_action);
    app.set_accels_for_action("app.open-folder", &["<Control><Shift>o"]);

    let toggle_sidebar_action = gio::SimpleAction::new("toggle-sidebar", None);
    {
        let split_view_slot = split_view_slot.clone();
        toggle_sidebar_action.connect_activate(move |_, _| {
            if let Some(view) = split_view_slot.borrow().as_ref() {
                view.set_collapsed(!view.is_collapsed());
            }
        });
    }
    app.add_action(&toggle_sidebar_action);
    app.set_accels_for_action("app.toggle-sidebar", &["<Control>b"]);

    {
        let split_view_slot = split_view_slot.clone();
        sidebar_toggle_btn.connect_clicked(move |_| {
            if let Some(view) = split_view_slot.borrow().as_ref() {
                view.set_collapsed(!view.is_collapsed());
            }
        });
    }

    // aparência: Sistema / Claro / Escuro (aplica em tempo real)
    {
        for (name, scheme) in [
            ("appearance-system", adw::ColorScheme::Default),
            ("appearance-light", adw::ColorScheme::ForceLight),
            ("appearance-dark", adw::ColorScheme::ForceDark),
        ] {
            let action = gio::SimpleAction::new(name, None);
            let style_manager = system_style.clone();
            let buffer = editor_buffer.clone();
            let source_style = source_style.clone();
            action.connect_activate(move |_, _| {
                style_manager.set_color_scheme(scheme);
                apply_source_scheme(&source_style, &style_manager, &buffer);
            });
            app.add_action(&action);
        }
    }

    let quit_action = gio::SimpleAction::new("quit", None);
    {
        let app = app.clone();
        quit_action.connect_activate(move |_, _| {
            app.quit();
        });
    }
    app.add_action(&quit_action);
    app.set_accels_for_action("app.quit", &["<Control>q", "<Control>w"]);

    app.add_action(&open_action);
    app.add_action(&save_action);
    app.add_action(&save_as_action);
    app.set_accels_for_action("app.open", &["<Control>o"]);
    app.set_accels_for_action("app.save", &["<Control>s"]);
    app.set_accels_for_action("app.save-as", &["<Control><Shift>s"]);

    // botões do header bar
    {
        let state = state.clone();
        let buffer = editor_buffer.clone();
        let window = window.clone();
        let window_title = window_title.clone();
        let toast_overlay = toast_overlay.clone();
        open_btn.connect_clicked(move |_| {
            open_markdown(
                window.clone(),
                buffer.clone(),
                state.clone(),
                window_title.clone(),
                toast_overlay.clone(),
            );
        });
    }
    {
        let state = state.clone();
        let buffer = editor_buffer.clone();
        let window = window.clone();
        let window_title = window_title.clone();
        let toast_overlay = toast_overlay.clone();
        save_btn.connect_clicked(move |_| {
            save_markdown(
                window.clone(),
                buffer.clone(),
                state.clone(),
                window_title.clone(),
                toast_overlay.clone(),
            );
        });
    }

    // ---- seed inicial + preview ao vivo ----
    editor_buffer.set_text(WELCOME);

    {
        let preview_box = preview_box.clone();
        let window = window.clone();
        let window_title = window_title.clone();
        let status_label = status_label.clone();
        let state = state.clone();
        editor_buffer.connect_changed(move |buf| {
            let (start, end) = buf.bounds();
            let src = buf.text(&start, &end, true);
            render_preview_widgets(&preview_box, &src);
            state.dirty.set(true);
            update_title(&window_title, &window, &state);
            update_status(&status_label, &src);
        });
    }

    // renderiza o conteúdo inicial sem marcar como "modificado"
    {
        let (start, end) = editor_buffer.bounds();
        let src = editor_buffer.text(&start, &end, true);
        render_preview_widgets(&preview_box, &src);
        state.dirty.set(false);
        update_title(&window_title, &window, &state);
        update_status(&status_label, &src);
    }

    // modo compacto: só os atalhos de teclado controlam a toolbar em janela estreita
    if let Ok(condition) = adw::BreakpointCondition::parse("max-width: 700px") {
        let bp = adw::Breakpoint::new(condition);
        bp.add_setter(&split_view, "collapsed", Some(&true.to_value()));
        window.add_breakpoint(bp);
    }

    if let Ok(condition) = adw::BreakpointCondition::parse("max-width: 460px") {
        let bp = adw::Breakpoint::new(condition);
        bp.add_setter(&open_btn, "visible", Some(&false.to_value()));
        bp.add_setter(&save_btn, "visible", Some(&false.to_value()));
        window.add_breakpoint(bp);
    }

    window.present();
}

fn choose_workspace_folder(
    window: &adw::ApplicationWindow,
    files_list: &gtk4::ListBox,
    indices_list: &gtk4::ListBox,
    folder_label: &gtk4::Label,
    parts: &Rc<AppParts>,
) {
    let dialog = gtk4::FileDialog::builder()
        .title("Abrir pasta (workspace)")
        .modal(true)
        .build();
    let w = window.clone();
    let fl = files_list.clone();
    let il = indices_list.clone();
    let folder_label = folder_label.clone();
    let parts = parts.clone();
    dialog.select_folder(
        Some(w.upcast_ref::<gtk4::Window>()),
        None::<&gio::Cancellable>,
        move |result| {
            let Ok(file) = result else {
                return; // cancelado
            };
            let Some(path) = file.path() else {
                eprintln!("uaiType: pasta sem path local");
                show_toast(&parts.toast_overlay, "Não foi possível abrir essa pasta.");
                return;
            };
            *parts.state.workspace.borrow_mut() = Some(path);
            refresh_workspace(&fl, &il, &folder_label, &parts);
            let name = parts
                .state
                .workspace
                .borrow()
                .clone()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
                .unwrap_or_else(|| "workspace".to_string());
            show_toast(&parts.toast_overlay, &format!("Workspace: {name}"));
        },
    );
}

fn show_toast(overlay: &adw::ToastOverlay, message: &str) {
    overlay.add_toast(adw::Toast::new(message));
}

fn update_status(status: &gtk4::Label, text: &str) {
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    status.set_label(&format!(
        "{words} palavras · {chars} caracteres · UTF-8 · Markdown"
    ));
}

fn update_title(
    window_title: &adw::WindowTitle,
    window: &adw::ApplicationWindow,
    state: &AppState,
) {
    let name = state
        .current_path
        .borrow()
        .as_ref()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "untitled.md".to_string());
    let dot = if state.dirty.get() { "● " } else { "" };
    window_title.set_title(&format!("{dot}{name}"));
    window_title.set_subtitle("uaiType");
    window.set_title(Some(&format!("{dot}{name} — uaiType")));
}

fn buffer_text(buffer: &sourceview::Buffer) -> String {
    let (start, end) = buffer.bounds();
    buffer.text(&start, &end, true).into()
}

fn open_file_direct(
    window: adw::ApplicationWindow,
    buffer: sourceview::Buffer,
    state: Rc<AppState>,
    path: PathBuf,
    window_title: adw::WindowTitle,
    toast: adw::ToastOverlay,
) {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "arquivo".to_string());
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            buffer.set_text(&content);
            *state.current_path.borrow_mut() = Some(path);
            state.dirty.set(false);
            update_title(&window_title, &window, &state);
            show_toast(&toast, &format!("Aberto: {name}"));
        }
        Err(err) => {
            eprintln!("uaiType: falha ao ler {}: {err}", path.display());
            show_toast(&toast, "Não foi possível abrir o arquivo.");
        }
    }
}

fn open_markdown(
    window: adw::ApplicationWindow,
    buffer: sourceview::Buffer,
    state: Rc<AppState>,
    window_title: adw::WindowTitle,
    toast: adw::ToastOverlay,
) {
    let dialog = gtk4::FileDialog::builder()
        .title("Abrir markdown")
        .modal(true)
        .build();
    let w = window.clone();
    let b = buffer.clone();
    let st = state.clone();
    let wt = window_title.clone();
    let t = toast.clone();
    dialog.open(Some(&w.clone()), None::<&gio::Cancellable>, move |result| {
        let Ok(file) = result else {
            return; // cancelado
        };
        let Some(path) = file.path() else {
            eprintln!("uaiType: arquivo sem path local");
            show_toast(&t, "Não foi possível abrir o arquivo.");
            return;
        };
        open_file_direct(w, b, st, path, wt, t);
    });
}

fn save_markdown(
    window: adw::ApplicationWindow,
    buffer: sourceview::Buffer,
    state: Rc<AppState>,
    window_title: adw::WindowTitle,
    toast: adw::ToastOverlay,
) {
    let path = state.current_path.borrow().clone();
    if let Some(path) = path {
        match std::fs::write(&path, buffer_text(&buffer)) {
            Ok(()) => {
                state.dirty.set(false);
                update_title(&window_title, &window, &state);
                show_toast(&toast, "Arquivo salvo.");
            }
            Err(err) => {
                eprintln!("uaiType: falha ao salvar {}: {err}", path.display());
                show_toast(&toast, "Não foi possível salvar o arquivo.");
            }
        }
    } else {
        save_markdown_as(window, buffer, state, window_title, toast);
    }
}

fn save_markdown_as(
    window: adw::ApplicationWindow,
    buffer: sourceview::Buffer,
    state: Rc<AppState>,
    window_title: adw::WindowTitle,
    toast: adw::ToastOverlay,
) {
    let dialog = gtk4::FileDialog::builder()
        .title("Salvar como")
        .modal(true)
        .initial_name("untitled.md")
        .build();
    let w = window.clone();
    let b = buffer.clone();
    let st = state.clone();
    let wt = window_title.clone();
    let t = toast.clone();
    dialog.save(Some(&w.clone()), None::<&gio::Cancellable>, move |result| {
        let Ok(file) = result else {
            return; // cancelado
        };
        let Some(path) = file.path() else {
            eprintln!("uaiType: arquivo sem path local");
            show_toast(&t, "Não foi possível salvar o arquivo.");
            return;
        };
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "untitled.md".to_string());
        match std::fs::write(&path, buffer_text(&b)) {
            Ok(()) => {
                *st.current_path.borrow_mut() = Some(path);
                st.dirty.set(false);
                update_title(&wt, &w, &st);
                show_toast(&t, &format!("Salvo como: {name}"));
            }
            Err(err) => {
                eprintln!("uaiType: falha ao salvar {}: {err}", path.display());
                show_toast(&t, "Não foi possível salvar o arquivo.");
            }
        }
    });
}

fn show_about(window: &adw::ApplicationWindow) {
    let about = adw::AboutDialog::builder()
        .application_name("uaiType")
        .application_icon("io.github.luanxis.uaitype")
        .developer_name("Luan SB")
        .version(env!("CARGO_PKG_VERSION"))
        .website("https://github.com/luanxis/uaitype")
        .issue_url("https://github.com/luanxis/uaitype/issues")
        .license_type(gtk4::License::MitX11)
        .comments(
            "Editor markdown nativo para Linux — Rust, GTK4 e libadwaita. \
             Sem webview, sem bloat: só a fonte e a visualização.",
        )
        .copyright("© 2026 Luan SB")
        .build();
    about.add_link("GitHub", "https://github.com/luanxis/uaitype");
    about.present(Some(window));
}

fn new_document(
    window: adw::ApplicationWindow,
    buffer: sourceview::Buffer,
    state: Rc<AppState>,
    window_title: adw::WindowTitle,
    status_label: gtk4::Label,
    toast: adw::ToastOverlay,
) {
    if state.dirty.get() {
        let dialog = adw::MessageDialog::builder()
            .transient_for(&window)
            .heading("Descartar alterações?")
            .body("O documento ainda não foi salvo. Continuar vai descartar o conteúdo atual.")
            .default_response("discard")
            .close_response("cancel")
            .modal(true)
            .build();
        dialog.add_response("cancel", "_Cancelar");
        dialog.add_response("discard", "_Descartar");
        dialog.set_response_appearance("discard", adw::ResponseAppearance::Destructive);
        let buffer = buffer.clone();
        let window = window.clone();
        let window_title = window_title.clone();
        let state = state.clone();
        // clone da janela porque o buffer é emprestado — o Excluding trait é Rc<Clone>-style
        let status_label = status_label.clone();
        let toast = toast.clone();
        dialog.connect_response(None, move |dialog, response| {
            if response == "discard" {
                buffer.set_text("");
                *state.current_path.borrow_mut() = None;
                state.dirty.set(false);
                update_title(&window_title, &window, &state);
                update_status(&status_label, "");
                show_toast(&toast, "Novo documento.");
            }
            dialog.close();
        });
        dialog.present();
        return;
    }
    buffer.set_text("");
    *state.current_path.borrow_mut() = None;
    state.dirty.set(false);
    update_title(&window_title, &window, &state);
    update_status(&status_label, "");
    show_toast(&toast, "Novo documento.");
}

// o SourceView não segue o tema do sistema sozinho — aplica o esquema claro/escuro

fn custom_scheme_manager() -> sourceview::StyleSchemeManager {
    let manager = sourceview::StyleSchemeManager::default();
    let mut candidates: Vec<String> =
        vec![concat!(env!("CARGO_MANIFEST_DIR"), "/res/schemes").to_string()];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("res/schemes").to_string_lossy().into_owned());
            candidates.push(
                parent
                    .join("../share/uaitype/schemes")
                    .to_string_lossy()
                    .into_owned(),
            );
        }
    }
    for dir in candidates {
        if std::path::Path::new(&dir).is_dir() {
            manager.append_search_path(&dir);
        }
    }
    manager.force_rescan();
    manager
}

fn apply_source_scheme(
    style_manager: &sourceview::StyleSchemeManager,
    system: &adw::StyleManager,
    buffer: &sourceview::Buffer,
) {
    let (custom_id, fallback_id) = if system.is_dark() {
        ("uaitype-dark", "Adwaita-dark")
    } else {
        ("uaitype-light", "Adwaita")
    };
    let scheme = style_manager
        .scheme(custom_id)
        .or_else(|| style_manager.scheme(fallback_id));
    if let Some(scheme) = scheme {
        buffer.set_style_scheme(Some(&scheme));
    }
}

// ===== workspace / sidebar =====

#[derive(Clone)]
#[allow(dead_code)] // status_label fica disponível para futuras features (ex.: duplo clique nos contadores)
struct AppParts {
    window: adw::ApplicationWindow,
    buffer: sourceview::Buffer,
    state: Rc<AppState>,
    window_title: adw::WindowTitle,
    status_label: gtk4::Label,
    toast_overlay: adw::ToastOverlay,
}

fn collect_tree(
    dir: &std::path::Path,
    depth: u32,
    out: &mut Vec<(u32, String, Option<PathBuf>, bool)>, // depth, label, full_path, is_folder
) {
    let mut entries: Vec<_> = match std::fs::read_dir(dir) {
        Ok(rd) => rd.flatten().collect(),
        Err(_) => return,
    };
    entries.sort_by_key(|e| {
        (
            !e.file_type().map(|t| t.is_dir()).unwrap_or(false),
            e.file_name(),
        )
    });
    for entry in entries {
        let ft = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        if ft.is_dir() {
            out.push((depth, name, None, true));
            collect_tree(&entry.path(), depth + 1, out);
        } else {
            let is_md =
                name.ends_with(".md") || name.ends_with(".markdown") || name.ends_with(".mkd");
            if is_md {
                out.push((depth, name, Some(entry.path()), false));
            }
        }
    }
}

fn collect_indices(dir: &std::path::Path, prefix: PathBuf, out: &mut Vec<(String, PathBuf)>) {
    let mut entries: Vec<_> = match std::fs::read_dir(dir) {
        Ok(rd) => rd.flatten().collect(),
        Err(_) => return,
    };
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let ft = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let rel = if prefix.as_os_str().is_empty() {
            PathBuf::from(&name)
        } else {
            prefix.join(&name)
        };
        if ft.is_dir() {
            collect_indices(&entry.path(), rel, out);
        } else {
            let is_md =
                name.ends_with(".md") || name.ends_with(".markdown") || name.ends_with(".mkd");
            if is_md {
                out.push((rel.to_string_lossy().into_owned(), entry.path()));
            }
        }
    }
}

fn add_entry_row(
    parts: &Rc<AppParts>,
    list: &gtk4::ListBox,
    depth: u32,
    label_text: &str,
    icon_name: &str,
    full_path: Option<PathBuf>,
) {
    let row = gtk4::ListBoxRow::new();
    let rbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    rbox.set_margin_start(4 + depth as i32 * 16);
    rbox.set_margin_end(6);
    rbox.set_margin_top(3);
    rbox.set_margin_bottom(3);
    let icon = gtk4::Image::from_icon_name(icon_name);
    icon.set_pixel_size(16);
    let label = gtk4::Label::new(Some(label_text));
    label.set_xalign(0.0);
    label.set_hexpand(true);
    label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    rbox.append(&icon);
    rbox.append(&label);

    match full_path {
        Some(p) => {
            let btn = gtk4::Button::new();
            btn.add_css_class("flat");
            btn.set_child(Some(&rbox));
            let parts = parts.clone();
            btn.connect_clicked(move |_| {
                open_file_direct(
                    parts.window.clone(),
                    parts.buffer.clone(),
                    parts.state.clone(),
                    p.clone(),
                    parts.window_title.clone(),
                    parts.toast_overlay.clone(),
                );
            });
            row.set_child(Some(&btn));
        }
        None => row.set_child(Some(&rbox)),
    }
    list.append(&row);
}

fn add_info_row(list: &gtk4::ListBox, text: &str) {
    let row = gtk4::ListBoxRow::new();
    row.set_activatable(false);
    row.set_selectable(false);
    let label = gtk4::Label::new(Some(text));
    label.set_xalign(0.0);
    label.add_css_class("dim-label");
    label.set_margin_start(12);
    label.set_margin_end(12);
    label.set_margin_top(10);
    label.set_margin_bottom(10);
    label.set_wrap(true);
    label.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
    row.set_child(Some(&label));
    list.append(&row);
}

fn clear_listbox(list: &gtk4::ListBox) {
    while let Some(row) = list.row_at_index(0) {
        list.remove(&row);
    }
}

fn refresh_workspace(
    files_list: &gtk4::ListBox,
    indices_list: &gtk4::ListBox,
    folder_label: &gtk4::Label,
    parts: &Rc<AppParts>,
) {
    clear_listbox(files_list);
    clear_listbox(indices_list);

    let root = parts.state.workspace.borrow().clone();
    match &root {
        None => {
            folder_label.set_label("Abrir pasta…");
            add_info_row(
                files_list,
                "Nenhuma pasta aberta. \\
              Clique no 📁 abaixo ou tecle Ctrl+Shift+O para escolher um vault.",
            );
            add_info_row(
                indices_list,
                "Sem workspace — os índices aparecem depois que você abrir uma pasta.",
            );
        }
        Some(root) => {
            let name = root
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "workspace".to_string());
            folder_label.set_label(&name);

            let mut rows: Vec<(u32, String, Option<PathBuf>, bool)> = Vec::new();
            collect_tree(root, 0, &mut rows);
            if rows.is_empty() {
                add_info_row(
                    files_list,
                    "Nenhum .md nesta pasta ainda. \\
                  Pressione Ctrl+N para criar um novo documento.",
                );
            } else {
                for (depth, text, full_path, is_folder) in rows {
                    let icon = if is_folder {
                        "folder-symbolic"
                    } else {
                        "text-x-generic-symbolic"
                    };
                    add_entry_row(parts, files_list, depth, &text, icon, full_path);
                }
            }

            let mut indices: Vec<(String, PathBuf)> = Vec::new();
            collect_indices(root, PathBuf::new(), &mut indices);
            if indices.is_empty() {
                add_info_row(
                    indices_list,
                    "Nenhum .md encontrado recursivamente nesta pasta.",
                );
            } else {
                for (rel, full) in indices {
                    add_entry_row(
                        parts,
                        indices_list,
                        0,
                        &rel,
                        "text-x-generic-symbolic",
                        Some(full),
                    );
                }
            }
        }
    }
}

fn unique_untitled(root: &std::path::Path) -> PathBuf {
    let base = root.join("untitled.md");
    if !base.exists() {
        return base;
    }
    for i in 1..100 {
        let candidate = root.join(format!("untitled-{i}.md"));
        if !candidate.exists() {
            return candidate;
        }
    }
    root.join(format!("untitled-{}.md", std::process::id()))
}

// ===== preview (widgets) =====

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[derive(Clone, Copy)]
enum Inline {
    Strong,
    Emphasis,
    Code,
    Strike,
    Link,
}

fn marks_with(marks: &[Inline], extra: Inline) -> Vec<Inline> {
    let mut m = Vec::with_capacity(marks.len() + 1);
    m.push(extra);
    m.extend_from_slice(marks);
    m
}

fn render_inline(marks: &[Inline], text: &str) -> String {
    let mut out = esc(text);
    for mark in marks.iter().rev() {
        out = match mark {
            Inline::Strong => format!("<b>{out}</b>"),
            Inline::Emphasis => format!("<i>{out}</i>"),
            Inline::Code => format!("<tt>{out}</tt>"),
            Inline::Strike => format!("<s>{out}</s>"),
            Inline::Link => format!("<u>{out}</u>"),
        };
    }
    out
}

fn flush_paragraph(
    container: &gtk4::Box,
    para: &mut String,
    heading: &mut Option<HeadingLevel>,
    quote: &bool,
) {
    if para.is_empty() {
        *heading = None;
        return;
    }
    let label = gtk4::Label::new(None);
    match heading {
        Some(HeadingLevel::H1) => label.add_css_class("preview-h1"),
        Some(HeadingLevel::H2) => label.add_css_class("preview-h2"),
        _ => label.add_css_class("preview-h3"),
    }
    if heading.is_none() {
        label.add_css_class("preview-p");
    }
    if *quote {
        label.add_css_class("preview-quote");
    }
    label.set_markup(para);
    label.set_wrap(true);
    label.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
    label.set_xalign(0.0);
    label.set_selectable(true);
    container.append(&label);
    para.clear();
    *heading = None;
}

const MERMAID_SCALE: f32 = 2.0;

// tema do diagrama acompanha o tema do app, com fundo transparente
fn mermaid_theme() -> mermaid_rs_renderer::Theme {
    let base = if adw::StyleManager::default().is_dark() {
        mermaid_rs_renderer::Theme::dark()
    } else {
        mermaid_rs_renderer::Theme::modern()
    };
    mermaid_rs_renderer::Theme {
        background: "transparent".to_string(),
        ..base
    }
}

fn mermaid_picture(code: &str) -> anyhow::Result<gtk4::Picture> {
    let options = mermaid_rs_renderer::RenderOptions {
        theme: mermaid_theme(),
        layout: mermaid_rs_renderer::LayoutConfig::default(),
    };
    let svg = mermaid_rs_renderer::render_with_options(code, options)?;
    let mut options = usvg::Options::default();
    // o Options default nasce com fontdb vazio — sem isso o texto vira nada
    options.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_data(svg.as_bytes(), &options)?;
    let size = tree.size();
    let w = (size.width() * MERMAID_SCALE).ceil() as u32;
    let h = (size.height() * MERMAID_SCALE).ceil() as u32;
    let mut pixmap = tiny_skia::Pixmap::new(w, h)
        .ok_or_else(|| anyhow::anyhow!("falha ao criar pixmap {w}x{h}"))?;
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(MERMAID_SCALE, MERMAID_SCALE),
        &mut pixmap.as_mut(),
    );
    let logical_h = (h as f32 / MERMAID_SCALE).ceil() as i32;
    let stride = (w * 4) as usize;
    let bytes = gtk4::glib::Bytes::from(pixmap.data());
    let texture = gtk4::gdk::MemoryTexture::new(
        w as i32,
        h as i32,
        gtk4::gdk::MemoryFormat::R8g8b8a8,
        &bytes,
        stride,
    );
    let picture = gtk4::Picture::for_paintable(&texture);
    picture.set_hexpand(true);
    picture.set_halign(gtk4::Align::Fill);
    picture.set_can_shrink(true);
    picture.set_size_request(0, logical_h);
    picture.add_css_class("preview-picture");
    Ok(picture)
}

fn render_mermaid_widget(container: &gtk4::Box, code: &str) {
    match mermaid_picture(code) {
        Ok(picture) => container.append(&picture),
        Err(err) => {
            let label =
                gtk4::Label::new(Some(&format!("{code}\n\n// mermaid indisponível: {err}")));
            label.add_css_class("preview-code");
            label.set_wrap(true);
            label.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
            label.set_xalign(0.0);
            label.set_selectable(true);
            container.append(&label);
        }
    }
}

fn clear_box(container: &gtk4::Box) {
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }
}

fn render_preview_widgets(container: &gtk4::Box, src: &str) {
    clear_box(container);

    const PARSER_OPTS: pulldown_cmark::Options = pulldown_cmark::Options::ENABLE_TABLES
        .union(pulldown_cmark::Options::ENABLE_STRIKETHROUGH)
        .union(pulldown_cmark::Options::ENABLE_TASKLISTS);

    let mut marks: Vec<Inline> = Vec::new();
    let mut para = String::new();
    let mut heading: Option<HeadingLevel> = None;
    let mut quote = false;
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_buf = String::new();
    let mut table: Option<gtk4::Grid> = None;
    let mut table_col: i32 = 0;
    let mut table_row: i32 = 0;

    let push_marked = |para: &mut String, marks: &[Inline], text: &str| {
        para.push_str(&render_inline(marks, text));
    };

    for event in Parser::new_ext(src, PARSER_OPTS) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                flush_paragraph(container, &mut para, &mut heading, &quote);
                heading = Some(level);
            }
            Event::End(TagEnd::Heading(_)) => {
                flush_paragraph(container, &mut para, &mut heading, &quote);
            }
            Event::Start(Tag::Paragraph) => {
                flush_paragraph(container, &mut para, &mut heading, &quote);
            }
            Event::End(TagEnd::Paragraph) | Event::End(TagEnd::Item) => {
                flush_paragraph(container, &mut para, &mut heading, &quote);
            }
            Event::Start(Tag::BlockQuote(_)) => quote = true,
            Event::End(TagEnd::BlockQuote(_)) => quote = false,
            Event::Start(Tag::List(_)) => {}
            Event::End(TagEnd::List(_)) => {}
            Event::Start(Tag::Item) => para.push_str("• "),
            Event::Start(Tag::Link { .. }) => marks.push(Inline::Link),
            Event::End(TagEnd::Link) => {
                if let Some(i) = marks.iter().rposition(|m| matches!(m, Inline::Link)) {
                    marks.remove(i);
                }
            }
            Event::Start(Tag::Strong) => marks.push(Inline::Strong),
            Event::End(TagEnd::Strong) => {
                if let Some(i) = marks.iter().rposition(|m| matches!(m, Inline::Strong)) {
                    marks.remove(i);
                }
            }
            Event::Start(Tag::Emphasis) => marks.push(Inline::Emphasis),
            Event::End(TagEnd::Emphasis) => {
                if let Some(i) = marks.iter().rposition(|m| matches!(m, Inline::Emphasis)) {
                    marks.remove(i);
                }
            }
            Event::Start(Tag::Strikethrough) => marks.push(Inline::Strike),
            Event::End(TagEnd::Strikethrough) => {
                if let Some(i) = marks.iter().rposition(|m| matches!(m, Inline::Strike)) {
                    marks.remove(i);
                }
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                flush_paragraph(container, &mut para, &mut heading, &quote);
                in_code_block = true;
                code_lang = match &kind {
                    pulldown_cmark::CodeBlockKind::Fenced(info) => info.to_string(),
                    pulldown_cmark::CodeBlockKind::Indented => String::new(),
                };
                code_buf.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                if code_lang.eq_ignore_ascii_case("mermaid") {
                    render_mermaid_widget(container, &code_buf);
                } else {
                    let frame = gtk4::Frame::new(None);
                    frame.add_css_class("preview-code");
                    let label = gtk4::Label::new(Some(esc(&code_buf).as_str()));
                    label.set_xalign(0.0);
                    label.set_selectable(true);
                    label.set_wrap(true);
                    label.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
                    frame.set_child(Some(&label));
                    container.append(&frame);
                }
            }
            Event::Start(Tag::Table(_)) => {
                flush_paragraph(container, &mut para, &mut heading, &quote);
                let grid = gtk4::Grid::new();
                grid.set_column_spacing(12);
                grid.set_row_spacing(6);
                grid.add_css_class("preview-table");
                table = Some(grid);
                table_row = 0;
                table_col = 0;
            }
            Event::End(TagEnd::Table) => {
                if let Some(grid) = table.take() {
                    container.append(&grid);
                }
            }
            Event::Start(Tag::TableRow) | Event::Start(Tag::TableHead) => {
                table_col = 0;
            }
            Event::End(TagEnd::TableRow) | Event::End(TagEnd::TableHead) => {
                table_row += 1;
            }
            Event::Start(Tag::TableCell) => {
                para.clear();
                marks.clear();
            }
            Event::End(TagEnd::TableCell) => {
                if let Some(grid) = table.as_ref() {
                    let label = gtk4::Label::new(None);
                    if !para.is_empty() {
                        label.set_markup(&para);
                    }
                    label.set_xalign(0.0);
                    label.set_wrap(true);
                    label.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
                    if table_row == 0 {
                        label.add_css_class("preview-cell-head");
                    } else {
                        label.add_css_class("preview-cell");
                    }
                    grid.attach(&label, table_col, table_row, 1, 1);
                }
                para.clear();
                table_col += 1;
            }
            Event::Text(text) => {
                if in_code_block {
                    code_buf.push_str(&text);
                } else {
                    push_marked(&mut para, &marks, &text);
                }
            }
            Event::Code(code) => {
                para.push_str(&render_inline(&marks_with(&marks, Inline::Code), &code))
            }
            Event::TaskListMarker(checked) => {
                para.push_str(if checked { "[x] " } else { "[ ] " });
            }
            Event::SoftBreak => para.push(' '),
            Event::HardBreak => para.push('\n'),
            Event::InlineHtml(html) => para.push_str(&esc(&html)),
            Event::Html(html) => para.push_str(&esc(&html)),
            Event::Rule => {
                container.append(&gtk4::Separator::new(gtk4::Orientation::Horizontal));
            }
            _ => {}
        }
    }
    // flush final (documento sem fechamento explícito)
    if in_code_block {
        // bloco de código não fechado — renderiza como código normal
        let label = gtk4::Label::new(Some(&esc(&code_buf)));
        label.add_css_class("preview-code");
        container.append(&label);
    } else {
        flush_paragraph(container, &mut para, &mut heading, &quote);
    }
    if let Some(grid) = table.take() {
        container.append(&grid);
    }
}
