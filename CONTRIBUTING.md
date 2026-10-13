# Contribuindo para o NotionPad Rust 🦀

O NotionPad é um laboratório de aprendizado comunitário: **gratuito e open source (MIT)**. Melhorias, relatórios de erro, documentação, traduções, testes e protótipos são bem-vindos.

## Antes de começar

- Veja as Issues e os Pull Requests existentes; se a mudança for grande, descreva a proposta em uma Issue antes de implementar.
- Atualmente priorizamos **Windows 10/11, Linux e Android**. macOS/iOS estão no roadmap, mas ainda não exigimos builds ou testes Apple.
- Prefira soluções com lógica em Rust, UI Slint e comportamento portável.
- Mantenha dependências limitadas e justifique novos pacotes (licença, tamanho, RAM, cold start).
- Não reintroduza um web runtime na interface sem uma discussão técnica na Issue.

## Enviando uma contribuição

1. Faça um fork e crie uma branch curta com o escopo da alteração.
2. Execute `cargo fmt --check` e `cargo test --lib` quando o ambiente de compilação estiver pronto; execute também `cargo clippy --all-targets -- -D warnings` se possível.
3. Explique no PR o que mudou e como testar. Inclua screenshots para mudanças de UI.
4. Para PRs que afirmem melhorar performance, compare antes/depois na mesma máquina e cenário; informe RAM, latência ou tempo de abertura.
5. Evite alterar formatos de arquivo sem testes de compatibilidade e cuidado para não perder dados.

## Princípios

**Corretude e integridade dos arquivos acima de benchmarks artificiais.** Não perder alterações, oferecer experiência fluida para documentos grandes, manter texto puro realmente puro e preservar portabilidade são prioridades.

## Licenciamento

Ao contribuir com código, você concorda que suas contribuições ao código próprio do NotionPad serão distribuídas sob a licença [MIT](LICENSE). Dependências (incluindo Slint) mantêm suas licenças.

Respeite as [condições de atribuição do Slint](https://slint.dev/terms-and-conditions) antes de publicar binários.
