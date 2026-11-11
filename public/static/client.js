// glow que segue o cursor (fundo, puramente decorativo)
const glow = document.getElementById('cursorGlow')
if (glow) {
  addEventListener('mousemove', (e) => {
    glow.style.transform = `translate(${e.clientX - 260}px, ${e.clientY - 260}px)`
  })
}

// o tema segue o sistema automaticamente, em tempo real
const systemTheme = window.matchMedia('(prefers-color-scheme: light)')
systemTheme.addEventListener('change', (e) => {
  document.documentElement.setAttribute('data-theme', e.matches ? 'light' : 'dark')
})

// camadas de movimento: só entram quando `html.fx` está ativo
// (desktop, ponteiro fine, sem prefers-reduced-motion — ver public/static/theme-init.js)
if (document.documentElement.classList.contains('fx')) {
  // janela do editor inclina com o cursor (tilt 3D, quantidade pequena)
  const win = document.querySelector('.win')
  if (win) {
    win.addEventListener('mousemove', (e) => {
      const r = win.getBoundingClientRect()
      const px = (e.clientX - r.left) / r.width - 0.5
      const py = (e.clientY - r.top) / r.height - 0.5
      win.style.transform = `perspective(1200px) rotateY(${px * 10}deg) rotateX(${py * -8}deg)`
    })
    win.addEventListener('mouseleave', () => {
      win.style.transform = ''
    })
  }

  // reveal ao rolar: adiciona .reveal + observa; adiciona .in ao entrar na viewport
  const targets = document.querySelectorAll(
    '.cards .card, .dl-btn, .price-box, .faq-item, section[id] .sec-label, section[id] h2'
  )
  const observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          entry.target.classList.add('in')
          observer.unobserve(entry.target)
        }
      }
    },
    { rootMargin: '0px 0px -8% 0px', threshold: 0.12 }
  )
  const groups = new Map()
  targets.forEach((el) => {
    const parent = el.parentElement ?? el
    const i = groups.get(parent) ?? 0
    groups.set(parent, i + 1)
    el.style.setProperty('--d', `${Math.min(i, 4) * 70}ms`)
    el.classList.add('reveal')
    observer.observe(el)
  })
}
