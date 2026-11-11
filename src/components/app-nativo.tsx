const APP_ITENS = [
  'Nativo de verdade: GTK4 + libadwaita, sem webview',
  'Preview renderizado na mesma janela — Ctrl+1 fonte · Ctrl+2 render',
  'Destaque de sintaxe markdown (SourceView5)',
  'Sidebar de workspace: árvore de pastas + índice plano de .md (Ctrl+B)',
  'Tema claro/escuro seguindo o sistema, via libadwaita',
  'Escrito em Rust, do zero, sem frameworks de UI pesados',
]

export const AppNativo = () => (
  <section id="app">
    <p class="sec-label">app nativo</p>
    <h2>Também é um app de verdade, no seu Linux.</h2>
    <div class="app-box">
      <div class="app-copy">
        <p>
          O uaiType agora existe como aplicativo nativo para Linux — uma janela GTK4 com
          editor e preview ao vivo, escrita em Rust. Sem Chromium embutido, sem electron;
          pesa pouco, abre rápido e segue o tema do seu sistema.
        </p>
        <div class="cta-row app-cta">
          <a href="#download" class="btn btn-primary">
            ↓ veja como rodar
          </a>
          <a href="https://github.com/luanxis/uaitype" target="_blank" rel="noopener" class="btn btn-outline">
            ver no GitHub
          </a>
        </div>
      </div>
      <ul class="price-list app-list">
        {APP_ITENS.map((item) => (
          <li>
            <span aria-hidden="true">✓</span> {item}
          </li>
        ))}
      </ul>
    </div>
  </section>
)
