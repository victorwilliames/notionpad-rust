# NotionPad Rust 🦀

Novo projeto experimental: um editor de notas e código focado em **Rust, performance mensurável, design moderno e multiplataforma**.

> Projeto independente, criado do zero. O antigo [NotionPad C#](https://github.com/victorwilliames/notionpad-csharp) é apenas referência de funcionalidades e UX.

## Status: primeiro editor utilizável (Etapas 0, 1 e 2 concluídas)

Já existem:
- **Múltiplas abas**: criação (+), alternância, fechamento seguro e indicador de alterações não salvas (`●`);
- **Motor de texto em Rust** com `Ropey`, Unicode estrito e cálculo em tempo real de linhas, palavras e caracteres;
- **Salvamento seguro e atômico** em UTF-8 preservando integridade contra interrupções de gravação;
- **Identidade Notion**: capas visuais sólidas e gradientes, controle de exibição, galeria de presets e botão para remoção;
- **Ícones de página**: seletor rápido de emojis Notion, posicionamento com sobreposição de capa e remoção de ícone;
- **Compatibilidade com metadados do NotionPad original**: leitura e persistência em `page_metadata.tsv`;
- **Interface compilada nativa em Slint**: inicialização rápida e sem runtime web (zero Electron/WebView);
- **Suíte de testes automatizados e Clippy** passando sem avisos (`cargo test` e `cargo clippy -D warnings`).

Ainda no roadmap das próximas etapas: editor rico/blocos avançados, syntax highlighting com parsing incremental e APK Android.

## Executar

Instale o Rust estável em <https://rustup.rs/> e rode:

```bash
cargo run
```

Testes do motor:

```bash
cargo test --lib
```

Build otimizado (desktop):

```bash
cargo build --release
```

É necessário ter as dependências de compilação gráfica do Slint e de diálogos nativos instaladas no Linux. Os targets Apple ficam para outra etapa, sem bloquearem Windows, Linux ou Android.

## Plataformas e prioridade

| Plataforma | Meta | Situação |
|---|---|---|
| Windows 10/11 | PRIORIDADE AGORA | Desktop: validar build, UX e benchmarks |
| Linux | PRIORIDADE AGORA | Desktop: validar build, UX e benchmarks |
| Android | PRIORIDADE AGORA | Implementar entrada Android, APK e UX por toque |
| macOS | DEPOIS | Manter arquitetura compatível; sem build ativo |
| iOS / iPadOS | DEPOIS | Manter arquitetura compatível; exigirá ecossistema Apple |

O código foi adicionado sem uma compilação local verificada até este ponto. Não há promessa de performance antes de benchmarks.

## Próximos passos

Ver [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) para decisões, limitações da UI temporária, critérios de performance, portabilidade e plano de evolução.

## Open source e comunidade

O NotionPad é **gratuito, open source e feito para colaboração**. É possível usar, estudar, fazer forks e enviar melhorias.
Quer participar? Leia [CONTRIBUTING.md](CONTRIBUTING.md), abra uma Issue para relatar erros ou propor recursos e envie Pull Requests.

Não existe paywall, assinatura ou plano pago planejado para o NotionPad.

## Licença

O **código próprio do NotionPad** usa [MIT](LICENSE): uma licença permissiva que permite uso, redistribuição e modificações, inclusive em projetos comerciais, desde que os avisos de licença sejam preservados. A MIT **não obriga** que forks permaneçam open source.

Dependências possuem licenças próprias. A interface usa **Slint** e, sob a licença Royalty-Free para desktop/mobile, a distribuição requer atribuição por `AboutSlint` ou badge no site público de distribuição. Vamos implementar essa atribuição antes da primeira release binária. Referência: <https://slint.dev/terms-and-conditions>.
