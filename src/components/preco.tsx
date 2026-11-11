const INCLUDED = [
  'Pré-visualização ao vivo (fonte + render)',
  'Markdown + extensões (GFM, Mermaid)',
  'Workspace: árvore de .md + índices (Ctrl+B)',
  'Linux (app nativo) — Windows/macOS em breve',
  'Sem conta, sem trial, sem licença paga',
  'Código aberto — contribua no GitHub',
]

export const Preco = () => (
  <section id="preco">
    <p class="sec-label">preço</p>
    <h2>Grátis, para sempre.</h2>
    <div class="price-box">
      <div class="price-main">
        <span class="price-amount">R$ 0</span>
        <p class="price-sub">
          para sempre · código aberto
          <br />
          sem pegadinha, sem asteriscos
        </p>
        <a href="https://github.com/luanxis/uaitype/releases/latest/download/uaitype.deb" class="btn btn-primary">
          ↓ baixar grátis
        </a>
      </div>
      <ul class="price-list">
        {INCLUDED.map((item) => (
          <li>
            <span aria-hidden="true">✓</span> {item}
          </li>
        ))}
      </ul>
    </div>
  </section>
)
