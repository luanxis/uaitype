export const DownloadSection = () => (
  <section id="download">
    <p class="sec-label">download</p>
    <h2>Rode no seu Linux</h2>
    <div class="dl-grid">
      <a class="dl-btn" href="https://github.com/luanxis/uaitype/releases/latest/download/uaitype.deb" target="_blank" rel="noopener" aria-label="Baixar o .deb do uaiType — Debian/Ubuntu">
        <span class="dl-ico">
          <svg viewBox="0 0 24 24" width="26" height="26" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <polyline points="4 17 10 11 4 5" />
            <line x1="12" y1="19" x2="20" y2="19" />
          </svg>
        </span>
        <span class="dl-platform">Linux (.deb)</span>
        <span class="dl-detail">Debian / Ubuntu — <code class="md-inline">apt install ./uaitype.deb</code> resolve dependências · GTK4 + Rust</span>
        <span class="dl-cta">baixar ↓</span>
      </a>
      <div class="dl-btn dl-soon" aria-disabled="true">
        <span class="dl-ico">
          <svg viewBox="0 0 24 24" width="26" height="26" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M18 3a3 3 0 0 0-3 3v12a3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3H6a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 3 3 0 0 0-3-3z" />
          </svg>
        </span>
        <span class="dl-platform">macOS</span>
        <span class="dl-detail">Em breve — sem previsão por enquanto</span>
        <span class="dl-cta">aguarde</span>
      </div>
      <div class="dl-btn dl-soon" aria-disabled="true">
        <span class="dl-ico">
          <svg viewBox="0 0 24 24" width="26" height="26" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <rect x="3" y="4" width="18" height="16" rx="2" />
            <line x1="12" y1="4" x2="12" y2="20" />
            <line x1="3" y1="12" x2="21" y2="12" />
          </svg>
        </span>
        <span class="dl-platform">Windows</span>
        <span class="dl-detail">Em breve — sem previsão por enquanto</span>
        <span class="dl-cta">aguarde</span>
      </div>
    </div>

    <div class="build-box">
      <p class="build-title">
        <span aria-hidden="true">$</span> instalado em 30 segundos
      </p>
      <pre class="build-code">{`# Debian / Ubuntu — resolve as dependências sozinho
wget https://github.com/luanxis/uaitype/releases/latest/download/uaitype.deb
sudo apt install ./uaitype.deb

# qualquer Linux (Arch, Fedora, openSUSE): tarball + script
wget https://github.com/luanxis/uaitype/releases/latest/download/uaitype-linux-x86_64.tar.gz
tar xzf uaitype-linux-x86_64.tar.gz
cd uaitype-linux-x86_64 && ./install.sh   # instala em ~/.local (use sudo p/ /usr/local)

# compilar do código-fonte
sudo apt install libgtk-4-dev libadwaita-1-dev libgtksourceview-5-dev cargo
git clone https://github.com/luanxis/uaitype.git
cd uaitype/app && cargo run`}</pre>
      <p class="dl-note">
        <span aria-hidden="true">&gt;</span> app em desenvolvimento ativo — a versão 0.1.0 está publicada em
        <a href="https://github.com/luanxis/uaitype/releases/latest" target="_blank" rel="noopener">Releases</a>.
        Melhor do que usar: contribuir de volta — veja o código no
        <a href="https://github.com/luanxis/uaitype" target="_blank" rel="noopener">
          <code class="md-inline">github.com/luanxis/uaitype</code></a>.
      </p>
    </div>
  </section>
)
