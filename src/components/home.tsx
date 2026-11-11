import { Nav } from './nav'
import { Hero } from './hero'
import { Features } from './features'
import { AppNativo } from './app-nativo'
import { DownloadSection } from './download'
import { Preco } from './preco'
import { Faq } from './faq'
import { Footer } from './footer'

export const Home = () => (
  <>
    <title>uaiType — um editor markdown minimalista</title>
    <meta
      name="description"
      content="uaiType é um editor markdown minimalista com live preview, disponível como app nativo GTK4 para Linux — grátis e de código aberto."
    />
    <meta property="og:title" content="uaiType — um editor markdown minimalista" />
    <meta
      property="og:description"
      content="Escreva em markdown, leia como documento. App nativo GTK4 para Linux, grátis e open source."
    />
    <meta property="og:type" content="website" />
    <meta media="(prefers-color-scheme: dark)" name="theme-color" content="#171512" />
    <meta media="(prefers-color-scheme: light)" name="theme-color" content="#FAF8F5" />

    <div class="bg-grid" aria-hidden="true"></div>
    <div class="cursor-glow" id="cursorGlow" aria-hidden="true"></div>
    <div class="scanlines" aria-hidden="true"></div>

    <Nav />

    <div class="wrap">
      <Hero />
    </div>

    <div class="wrap">
      <Features />
      <AppNativo />
      <DownloadSection />
      <Preco />
      <Faq />
    </div>

    <Footer />
  </>
)
