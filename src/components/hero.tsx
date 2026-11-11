export const Hero = () => (
  <header class="hero" id="inicio">
    <div>
      <p class="kicker">editor markdown minimalista</p>
      <h1>
        Escreva em markdown,
        <br />
        leia como{' '}
        <span class="glitch" data-text="documento.">
          documento.
        </span>
      </h1>
      <p class="subtitle">
        O uaiType é um editor markdown nativo para Linux — fonte com highlight e o documento
        renderizado na mesma janela, com pasta de trabalho (árvore + índices) para organizar as
        suas notas. Grátis e de código aberto.
      </p>
      <div class="cta-row">
        <a href="#download" class="btn btn-primary">
          ↓ baixar grátis
        </a>
        <a href="https://github.com/luanxis/uaitype" target="_blank" rel="noopener" class="btn btn-outline">
          $ ver no GitHub
        </a>
      </div>
      <div class="mini-stats">
        <span>
          onde: <b>Linux</b>
        </span>
        <span>
          licença: <b>open source</b>
        </span>
        <span>
          preço: <b>R$ 0</b>
        </span>
      </div>
    </div>
    <div>
      <div class="win editor-win" aria-hidden="true">
        <div class="win-bar">
          <span class="d r"></span>
          <span class="d y"></span>
          <span class="d g"></span>
          <span class="title">~/documentos/bem-vindo.md</span>
        </div>
        <div class="win-body editor-body">
          <h2 class="md-h1">Bem-vindo ao uaiType</h2>
          <p class="md-p">
            Este texto foi <strong class="md-strong">renderizado enquanto você digita</strong> — sem
            preview separado, sem trocar de modo.
          </p>
          <ul class="md-tasks">
            <li class="done">
              <span class="box">✓</span> Escrever no fluxo, não na sintaxe
            </li>
            <li class="done">
              <span class="box">✓</span> Tabelas, listas e{' '}
              <code class="md-inline">code fences</code> na hora
            </li>
            <li>
              <span class="box empty"></span> Exportar o rascunho como PDF
            </li>
          </ul>
          <pre class="md-code"><span class="tok-kw">const</span> ideias = <span class="tok-str">'solta agora'</span><span class="caret"></span></pre>
        </div>
      </div>
      <p class="win-caption">
        live preview — <b>o que você digita é o que você lê</b>
      </p>
    </div>
  </header>
)
