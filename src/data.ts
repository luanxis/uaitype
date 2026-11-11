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
      'Um editor markdown nativo para Linux, escrito em Rust com GTK4 e libadwaita — sem webview. Fonte com highlight, preview GFM com tabelas e mermaid na mesma janela, e workspace de pastas para organizar as notas. A experiência WYSIWYG completa é a Fase 2.',
  },
  {
    question: 'É mesmo grátis?',
    answer:
      'É. O uaiType é de código aberto (MIT) e gratuito para sempre — sem trial, sem licença paga, sem pegadinha. Se ele for útil para você, a melhor forma de retribuir é contribuir no GitHub ou indicar para alguém.',
  },
  {
    question: 'Quais plataformas são suportadas?',
    answer:
      'Linux, como app nativo. Para Debian/Ubuntu, o .deb na seção Download instala com um clique; nas demais distros, o tarball acompanha um install.sh. Windows e macOS estão no horizonte, assim que o app amadurecer.',
  },
  {
    question: 'Como eu rodo o app agora?',
    answer:
      'Baixe o .deb (ou o tarball) na seção Download — não precisa compilar. Se preferir rodar do código: `sudo apt install libgtk-4-dev libadwaita-1-dev libgtksourceview-5-dev cargo && git clone https://github.com/luanxis/uaitype.git && cd uaitype/app && cargo run`.',
  },
  {
    question: 'Posso contribuir com o projeto?',
    answer:
      'Com certeza. O uaiType é open source: issues, pull requests e feedback são bem-vindos no repositório do GitHub.',
  },
  {
    question: 'O uaiType está pronto?',
    answer:
      'A versão 0.1.0 está publicada — download na seção Download. O projeto está em desenvolvimento ativo: há arestas e a Fase 2 (WYSIWYG inline) não saiu ainda. Issues e PRs são sempre bem-vindos.',
  },
]
