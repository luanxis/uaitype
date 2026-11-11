import { base } from '../base'

export const Footer = () => (
  <footer>
    <div class="footer-inner">
      <a class="logo footer-logo" href="#inicio">
        <img class="pixel" src={`${base}static/uaitype.svg`} alt="Logo do uaiType" width="28" height="28" />
        uai<span>Type</span>
      </a>
      <nav class="footer-links" aria-label="Links do projeto">
        <a href="https://github.com/luanxis/uaitype" target="_blank" rel="noopener">
          github
        </a>
        <a href="https://github.com/luanxis/uaitype/issues" target="_blank" rel="noopener">
          issues
        </a>
        <a href="#app">app</a>
        <a href="#download">download</a>
      </nav>
    </div>
    <p>
      © 2026 uaiType — <span class="hash">MIT</span> · feito com Hono e publicado no GitHub Pages
    </p>
  </footer>
)
