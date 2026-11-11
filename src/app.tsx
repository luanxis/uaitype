import { Hono } from 'hono'
import { renderer } from './renderer'
import { Home } from './components/home'
import { NotFound, ServerError } from './components/errors'

const app = new Hono()

// Site estático (GitHub Pages): o build pré-renderiza estas rotas.
// Veja scripts/prerender.mjs. Segurança/headers ficam no <meta> CSP do renderer,
// porque o Pages não injeta headers de resposta.

app.use(renderer)

app.get('/', (c) => c.render(<Home />))

// página 404 como rota 200 — o Pages precisa de 404.html na raiz
app.get('/404', (c) => c.render(<NotFound />))

app.get('/health', (c) =>
  c.json({ status: 'ok', service: 'uaitype', build: process.env.UAITYPE_BUILD ?? 'dev' })
)

app.notFound((c) => {
  c.status(404)
  return c.render(<NotFound />)
})

app.onError((err, c) => {
  console.error(`${c.req.method} ${c.req.path}:`, err)
  c.status(500)
  return c.render(<ServerError />)
})

export default app
