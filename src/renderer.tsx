import { jsxRenderer } from 'hono/jsx-renderer'
import { base } from './base'

// CSP completa vira <meta> (o GitHub Pages não injeta headers de resposta)
const CSP = [
  "default-src 'self'",
  "script-src 'self'",
  "style-src 'self' 'unsafe-inline' https://fonts.googleapis.com",
  'font-src https://fonts.gstatic.com',
  "img-src 'self' data:",
  "connect-src 'self'",
  "object-src 'none'",
  "base-uri 'self'",
  "form-action 'self'",
].join('; ')

export const renderer = jsxRenderer(({ children }) => (
  <html lang="pt-BR">
    <head>
      <meta charset="UTF-8" />
      <meta http-equiv="Content-Security-Policy" content={CSP} />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <script src={`${base}static/theme-init.js`}></script>
      <link rel="icon" type="image/svg+xml" href={`${base}static/uaitype.svg`} />
      <link rel="preconnect" href="https://fonts.googleapis.com" />
      <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
      <link
        href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=JetBrains+Mono:wght@400;600&display=swap"
        rel="stylesheet"
      />
      <link rel="stylesheet" href={`${base}static/style.css`} />
    </head>
    <body>
      {children}
      <script src={`${base}static/client.js`}></script>
    </body>
  </html>
))
