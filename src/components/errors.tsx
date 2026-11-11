import type { PropsWithChildren } from 'hono/jsx'

const ErrorShell = (
  props: PropsWithChildren<{
    code: string
    title: string
    description: string
  }>
) => (
  <>
    <title>{props.code} — uaiType</title>
    <meta name="robots" content="noindex" />
    <div class="bg-grid" aria-hidden="true"></div>
    <div class="cursor-glow" id="cursorGlow" aria-hidden="true"></div>
    <div class="scanlines" aria-hidden="true"></div>

    <main class="wrap error-wrap">
      <section class="win error-win" aria-labelledby="error-title">
        <div class="win-body error-body">
          <div class="error-grid">
            <div class="error-code-box" aria-hidden="true">
              <span class="error-code glitch" data-text={props.code}>
                {props.code}
              </span>
              <span class="error-sub">{props.code === '404' ? 'NOT_FOUND' : 'INTERNAL_ERROR'}</span>
            </div>

            <div class="error-copy">
              <h1 id="error-title" class="error-title">
                {props.title}
              </h1>
              <p class="error-msg">{props.description}</p>
            </div>
          </div>

          {props.children}
        </div>
      </section>
    </main>
  </>
)

export const NotFound = () => (
  <ErrorShell
    code="404"
    title="Essa rota não existe."
    description="O endereço que você tentou abrir não está no filesystem. Pode ter sido um typo no path ou um link antigo apontando para algo que mudou."
  >
    <div class="error-actions">
      <a href="/" class="btn btn-primary">
        $ cd ~/
      </a>
    </div>
  </ErrorShell>
)

export const ServerError = () => (
  <ErrorShell
    code="500"
    title="Algo falhou do meu lado."
    description="O handler quebrou antes de responder. O stack trace já foi capturado no log do servidor; nada saiu do seu lado."
  >
    <div class="error-actions">
      <a href="/" class="btn btn-primary">
        $ cd ~/
      </a>
    </div>
  </ErrorShell>
)
