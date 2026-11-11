import { FEATURES, type FeatureIcon } from '../data'

const Icon = ({ name }: { name: FeatureIcon }) => {
  switch (name) {
    case 'live':
      return (
        <svg viewBox="0 0 24 24" width="30" height="30" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
        </svg>
      )
    case 'markdown':
      return (
        <svg viewBox="0 0 24 24" width="30" height="30" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
          <polyline points="14 2 14 8 20 8" />
          <line x1="9" y1="13" x2="15" y2="13" />
          <line x1="9" y1="17" x2="13" y2="17" />
        </svg>
      )
    case 'focus':
      return (
        <svg viewBox="0 0 24 24" width="30" height="30" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <circle cx="12" cy="12" r="7" />
          <circle cx="12" cy="12" r="1.5" fill="currentColor" stroke="none" />
          <line x1="12" y1="1" x2="12" y2="5" />
          <line x1="12" y1="19" x2="12" y2="23" />
          <line x1="1" y1="12" x2="5" y2="12" />
          <line x1="19" y1="12" x2="23" y2="12" />
        </svg>
      )
    case 'workspace':
      return (
        <svg viewBox="0 0 24 24" width="30" height="30" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <rect x="3" y="4" width="18" height="16" rx="2" />
          <line x1="10" y1="4" x2="10" y2="20" />
          <line x1="6" y1="9" x2="7.5" y2="9" />
          <line x1="6" y1="13" x2="7.5" y2="13" />
        </svg>
      )
  }
}

export const Features = () => (
  <section id="features">
    <p class="sec-label">recursos</p>
    <h2>Simples na superfície. Poderoso por dentro.</h2>
    <div class="cards">
      {FEATURES.map((feature, i) => (
        <div class="card">
          <div class="head">
            <span class="ico">
              <Icon name={feature.icon} />
            </span>
            <span class="num">{String(i + 1).padStart(2, '0')}</span>
          </div>
          <h3>{feature.name}</h3>
          <p>{feature.desc}</p>
          <span class="tag">{feature.tag}</span>
        </div>
      ))}
    </div>
  </section>
)
