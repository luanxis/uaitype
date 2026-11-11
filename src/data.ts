export type FeatureIcon = 'live' | 'markdown' | 'focus' | 'workspace'

export type Feature = {
  icon: FeatureIcon
  name: string
  desc: string
  tag: string
}

export const FEATURES: Feature[] = [
  {
    icon: 'live',
    name: 'Live preview',
    desc: 'Ctrl+1 edita o markdown, Ctrl+2 mostra o documento renderizado — GFM, tabelas e mermaid na hora, na mesma janela.',
    tag: 'fonte · preview · Ctrl+1/2',
  },
  {
    icon: 'markdown',
    name: 'Markdown e extensões',
    desc: 'Tabelas GFM, code fences com syntax highlight, diagramas Mermaid e task lists. O essencial, sem configuração infinita.',
    tag: 'GFM · Mermaid',
  },
  {
    icon: 'workspace',
    name: 'Workspace de notas',
    desc: 'Abra uma pasta e navegue pela árvore de .md: a aba Arquivos mostra a hierarquia, a aba Índices lista todos os markdowns de uma vez. Ctrl+B esconde a sidebar.',
    tag: 'sidebar · árvore · índices',
  },
]

export const FAQS: Array<{ question: string; answer: string }> = [
  {
    question: 'O que é o uaiType?',
    answer:
      'Um editor markdown minimalista com live preview — e que agora também é um app nativo para Linux, escrito em Rust com GTK4 e libadwaita, sem webview. A experiência completa de WYSIWYG é a Fase 2 do roadmap.',
  },
  {
    question: 'É mesmo grátis?',
    answer:
      'É. O uaiType é de código aberto (MIT) e gratuito para sempre — sem trial, sem licença paga, sem pegadinha. Se ele for útil para você, a melhor forma de retribuir é contribuir no GitHub ou indicar para alguém.',
  },
  {
    question: 'Quais plataformas são suportadas?',
    answer:
      'Hoje o foco é Linux, com o app nativo (GTK4 + libadwaita) pronto para compilar do código-fonte. Windows e macOS estão no horizonte, assim que o app amadurecer.',
  },
  {
    question: 'Como eu rodo o app agora?',
    answer:
      'Clone o repositório, instale os pacotes GTK4 (veja a tabela por distro na seção de download) e rode `cd app && cargo run`. As instruções completas estão no app/README.md do repositório — o processo leva poucos minutos.',
  },
  {
    question: 'Posso contribuir com o projeto?',
    answer:
      'Com certeza. O uaiType é open source: issues, pull requests e feedback são bem-vindos no repositório do GitHub.',
  },
  {
    question: 'O uaiType está pronto?',
    answer:
      'Ainda não — o projeto está em desenvolvimento ativo. O app Linux já existe como código-fonte; os downloads prontos para uso serão publicados aqui quando as primeiras builds saírem do forno.',
  },
]
