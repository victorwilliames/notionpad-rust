# NotionPad Rust — ROADMAP ALPHA (10 ETAPAS)

> **Objetivo:** a Alpha está concluída somente ao finalizar a **ETAPA 10** com release pública **v0.1.0-alpha**. Tudo listado abaixo começa como pendente e deve ser marcado concluído apenas com testes e evidências.

## Princípios e contexto

- **100% gratuito e open source**; licença MIT para código próprio; cumprir licenças de terceiros, incluindo Slint.
- **Plataformas Alpha:** Windows 10/11, Linux e Android. **Depois:** macOS, iOS e iPadOS. Preservar portabilidade da lógica Rust.
- **Trabalho direto na branch `main`**: commit e push sem PRs/branches extras; preservar histórico e alterações locais, nunca usar force push.
- **Código C# apenas referência:** `C:\.PROJETOS - Sites 2026\clonagem\notionpad-csharp` — o próprio Antigravity criou o original e pode executar, depurar e comparar, mas **não modificá-lo**.
- **Projeto novo:** https://github.com/victorwilliames/notionpad-rust. Aproveitar fundação Slint+Ropey existente e conferir o estado atual da main antes de alterar.
- Prioridade: **motor de edição Rust correto, incremental e rápido**, integridade de dados, UX excepcional e performance **medida**, não presumida.
- Metas como cold start <300 ms e RAM ~50 MB são exploratórias e dependem do hardware; jamais garantir sem benchmark.

## Resumo e sequência

| Etapa | GitHub Issue | Marco |
|---|---|---|
| 01 | [#2](https://github.com/victorwilliames/notionpad-rust/issues/2) | Fundação Rust e ambiente |
| 02 | [#3](https://github.com/victorwilliames/notionpad-rust/issues/3) | Core multiplataforma e arquivos |
| 03 | [#4](https://github.com/victorwilliames/notionpad-rust/issues/4) | Motor de edição incremental |
| 04 | [#5](https://github.com/victorwilliames/notionpad-rust/issues/5) | Editor desktop diário |
| 05 | [#6](https://github.com/victorwilliames/notionpad-rust/issues/6) | Rich text e Markdown |
| 06 | [#7](https://github.com/victorwilliames/notionpad-rust/issues/7) | Realce de sintaxe e modo código |
| 07 | [#8](https://github.com/victorwilliames/notionpad-rust/issues/8) | Redesign e personalização |
| 08 | [#9](https://github.com/victorwilliames/notionpad-rust/issues/9) | Windows, Linux e Android |
| 09 | [#10](https://github.com/victorwilliames/notionpad-rust/issues/10) | Profiling, testes e estabilidade |
| 10 | [#11](https://github.com/victorwilliames/notionpad-rust/issues/11) | Release Alpha v0.1.0 |

---
## ETAPA 01 — Fundação Rust e ambiente

**Issue:** [#2](https://github.com/victorwilliames/notionpad-rust/issues/2)

- [ ] Consolidar a fundação Rust + Slint + Ropey na main sem perder trabalho atual
- [ ] Inspecionar NotionPad C# local como referência, somente leitura
- [ ] Compilar, executar e corrigir o protótipo Windows/Linux
- [ ] Configurar toolchain, fmt, clippy, cargo test, build release e documentação

**Critério de conclusão:** Editor abre, edita e salva; compilação e testes documentados.

---

## ETAPA 02 — Core multiplataforma e arquivos

**Issue:** [#3](https://github.com/victorwilliames/notionpad-rust/issues/3)

- [ ] Separar core Rust de UI e sistema operacional
- [ ] Modelar documento, abas, dirty state e erros
- [ ] Criar abertura/salvamento seguro; preservar Unicode, BOM, CRLF/LF e newlines
- [ ] Persistir capas/emoji sem contaminar TXT/código; planejar formato rico versionado
- [ ] Testar round-trips e falhas de I/O

**Critério de conclusão:** Core portátil e arquivos preservados sem alteração involuntária.

---

## ETAPA 03 — Motor de edição incremental

**Issue:** [#4](https://github.com/victorwilliames/notionpad-rust/issues/4)

- [ ] Implementar buffer Rust por edições incrementais, avaliar Ropey
- [ ] Cursor, seleção, graphemes, IME, undo/redo
- [ ] Virtualização/viewport; não copiar ou redesenhar documento inteiro por tecla
- [ ] Benchmark com textos 1/10/100 MB e testes de integridade

**Critério de conclusão:** Digitação e seleção corretas, trabalho incremental demonstrado em testes.

---

## ETAPA 04 — Editor desktop diário

**Issue:** [#5](https://github.com/victorwilliames/notionpad-rust/issues/5)

- [ ] Abas múltiplas, criar/abrir/salvar/salvar como/fechar
- [ ] Atalhos, confirmação de mudanças, drag-and-drop
- [ ] Busca e substituição eficiente; status linha/coluna/caracteres
- [ ] Zoom, preferências de fontes, foco e HiDPI

**Critério de conclusão:** Editor utilizável com múltiplos documentos e salvamento confiável.

---

## ETAPA 05 — Rich text e Markdown

**Issue:** [#6](https://github.com/victorwilliames/notionpad-rust/issues/6)

- [ ] Negrito/itálico/sublinhado/tachado/marca-texto
- [ ] H1–H3, listas, citações, links, código inline e blocos
- [ ] Toolbar contextual e atalhos Markdown
- [ ] Formato rico versionado com serialização confiável
- [ ] Manter TXT/código puros e testar persistência de estilos

**Critério de conclusão:** Estilos sobrevivem a salvar/reabrir; sem corrupção de Markdown e texto puro.

---

## ETAPA 06 — Realce de sintaxe e modo código

**Issue:** [#7](https://github.com/victorwilliames/notionpad-rust/issues/7)

- [ ] Detectar linguagem e permitir seleção manual
- [ ] Suportar Rust, HTML, CSS, JS, TS, JSON, XML, Python, C#, SQL, Markdown
- [ ] Analisar Tree-sitter/parsing incremental; destacar tokens sem alterar buffer
- [ ] Números de linha, indentação e testes com documentos grandes

**Critério de conclusão:** Syntax highlighting correto, incremental e sem modificar conteúdo.

---

## ETAPA 07 — Redesign e personalização

**Issue:** [#8](https://github.com/victorwilliames/notionpad-rust/issues/8)

- [ ] Criar design system editorial, dark mode e tokens para light mode
- [ ] Redesenhar tabs, topbar, statusbar, menus, popovers e toolbar inline
- [ ] Implementar capas sólidas/gradientes/galeria/upload/URL, reposicionamento
- [ ] Implementar emojis/ícones, cache de imagens, animações breves e acessibilidade
- [ ] Persistir e reabrir metadados e comparar com app C#

**Critério de conclusão:** Visual superior ao original, com baixo custo e metadados persistentes.

---

## ETAPA 08 — Windows, Linux e Android

**Issue:** [#9](https://github.com/victorwilliames/notionpad-rust/issues/9)

- [ ] Validar Windows 10/11 e Linux em ambientes gráficos reais
- [ ] Configurar SDK/NDK, targets Rust, entrypoint Android e APK
- [ ] Adaptar UX para toque, teclado virtual, seleção e telas pequenas
- [ ] Integrar Storage Access Framework, permissões e ciclo de vida
- [ ] Executar smoke tests por plataforma e criar CI

**Critério de conclusão:** Windows/Linux testados e APK Android capaz de abrir, editar e salvar.

---

## ETAPA 09 — Profiling, testes e estabilidade

**Issue:** [#10](https://github.com/victorwilliames/notionpad-rust/issues/10)

- [ ] Medir cold/warm start, RAM/CPU/GPU, p50/p95/p99 da digitação e rolagem
- [ ] Comparar na mesma máquina com NotionPad C# e Notepad
- [ ] Testar arquivos 1/10/100 MB, parser, busca, Unicode, IME, recover e I/O
- [ ] Corrigir gargalos e bugs críticos; auditar licenças/segurança
- [ ] Executar fmt, clippy, testes e builds release

**Critério de conclusão:** Resultados reproduzíveis publicados, ausência de bugs críticos bloqueadores.

---

## ETAPA 10 — Release Alpha v0.1.0

**Issue:** [#11](https://github.com/victorwilliames/notionpad-rust/issues/11)

- [ ] Conferir aceite das etapas 01–09 e congelar escopo
- [ ] Smoke tests end-to-end de abas, rich text, código, capas, busca, salvar/reabrir
- [ ] Publicar artefatos Windows, Linux e APK Android verificados
- [ ] Finalizar README, CONTRIBUTING, changelog, checksums, licenças, atribuição Slint
- [ ] Criar tag GitHub e release pre-release e registrar limitações reais

**Critério de conclusão:** v0.1.0-alpha pública, gratuita, open source e funcional nas três plataformas prioritárias.

## Regras de execução para o Antigravity

1. Verificar o **estado atual** de `main` antes de começar; preservar tudo que já existe.
2. Fazer uma etapa por vez, admitindo trabalhos preparatórios em paralelo apenas quando seguros.
3. Para cada tarefa: implementar, testar, documentar evidências, commit e push na `main`.
4. Encerrar cada issue somente ao cumprir seus checklists e critérios de conclusão. Se faltar hardware/build de uma plataforma, registrar como não validada.
5. Não usar `git push --force`, não criar PRs ou branches e não reescrever código funcional à toa.
6. Manter o aplicativo anterior C# intacto. No novo Rust, preservar arquivos, Unicode, formatações e metadados.
7. Não fingir que o `TextEdit` provisório ou Ropey sozinho são o motor de edição final; virtualização, renderização e eventos incrementais devem ser reais.
8. **Alpha concluída = issue #11 cumprida, tag/release publicada e artefatos testados**, não apenas interface visual pronta.
