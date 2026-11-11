import { base } from '../base'

export const Nav = () => (
  <nav class="site-nav">
    <a class="logo" href="#inicio">
      <img class="pixel" src={`${base}static/uaitype.svg`} alt="Logo do uaiType" width="34" height="34" />
      uai<span>Type</span>
    </a>
    <ul class="nav-links">
      <li>
        <a href="#features">features</a>
      </li>
      <li>
        <a href="#app">app</a>
      </li>
      <li>
        <a href="#preco">preço</a>
      </li>
      <li>
        <a href="#faq">faq</a>
      </li>
    </ul>
    <a class="btn btn-primary nav-cta" href="https://github.com/luanxis/uaitype/releases/latest/download/uaitype.deb">
      ↓ download
    </a>
  </nav>
)
