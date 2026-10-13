# NotionPad Rust 🦀

Novo projeto experimental: um editor de notas e código focado em **Rust, performance mensurável, design moderno e multiplataforma**.

> Projeto independente, criado do zero. O antigo [NotionPad C#](https://github.com/victorwilliames/notionpad-csharp) é apenas referência de funcionalidades e UX.

## Status: fundação / protótipo desktop

Já existem:
- motor de texto Rust com `Ropey` e testes para Unicode/edição;
- janela Slint (sem Electron, WebView, HTML ou JavaScript);
- edição inicial de texto, abrir arquivo, salvar arquivo UTF-8 e proteção simples contra descarte acidental de alterações.

Ainda **não** existem: editor de texto rico, syntax highlighting, múltiplas abas, capas, ícones, virtualização, mobile ou benchmarking validado. O campo TextEdit inicial é temporário.

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

É necessário ter as dependências de compilação gráfica do Slint e de diálogos nativos instaladas no Linux. Windows/macOS usam os recursos do sistema conforme o framework.

## Plataformas planejadas

| Plataforma | Meta | Situação |
|---|---|---|
| Windows 10/11 | Executável nativo | Protótipo ainda sem validação de build |
| macOS | Aplicativo nativo | Protótipo ainda sem validação de build |
| Linux | Executável nativo | Protótipo ainda sem validação de build |
| Android | APK/AAB com lógica Rust | Planejado, requer entrada Android |
| iOS / iPadOS | App com lógica Rust | Planejado, requer macOS + Xcode |

O código foi adicionado sem uma compilação local verificada até este ponto. Não há promessa de performance antes de benchmarks.

## Próximos passos

Ver [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) para decisões, limitações da UI temporária, critérios de performance, portabilidade e plano de evolução.

## Licença

Código próprio: MIT. O Slint possui licenças próprias (inclusive royalty-free com atribuição para desktop/mobile). Leia as condições antes de distribuir.
