export const DownloadSection = () => (
  <section id="download">
    <p class="sec-label">download</p>
    <h2>Rode no seu Linux</h2>
    <div class="dl-grid">
      <a class="dl-btn" href="https://github.com/luanxis/uaitype" target="_blank" rel="noopener" aria-label="Ver o código do uaiType no GitHub">
        <span class="dl-ico">
          <svg viewBox="0 0 24 24" width="26" height="26" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <polyline points="4 17 10 11 4 5" />
            <line x1="12" y1="19" x2="20" y2="19" />
          </svg>
        </span>
        <span class="dl-platform">Linux (nativo)</span>
        <span class="dl-detail">App GTK4 + Rust — clone e compile com o cargo</span>
        <span class="dl-cta">ver código →</span>
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
        <span aria-hidden="true">$</span> como rodar na sua máquina
      </p>
      <pre class="build-code">{`# Debian / Ubuntu
sudo apt install libgtk-4-dev libadwaita-1-dev libgtksourceview-5-dev cargo

# Fedora
sudo dnf install gtk4-devel libadwaita-devel gtksourceview5-devel cargo

# clone e rode
git clone https://github.com/luanxis/uaitype.git
cd uaitype/app && cargo run`}</pre>
      <p class="dl-note">
        <span aria-hidden="true">&gt;</span> app em desenvolvimento ativo — sem binários publicados
        por enquanto; o jeito oficial é compilar do código-fonte (veja{' '}
        <code class="md-inline">app/README.md</code> no repositório). Arch? veja a tabela no README.
      </p>
    </div>
  </section>
)
