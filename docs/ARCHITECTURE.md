# NotionPad Rust: arquitetura e decisões (laboratório)

## Objetivo
Editor de notas e código, refeito em Rust, com design moderno, alta responsividade e uma base compartilhável entre Windows, macOS, Linux, Android e iOS. O NotionPad C# anterior serve SOMENTE de referência de produto.

## Stack escolhida para o primeiro experimento
- Rust 2024: lógica, arquivos, modelos e testes.
- Slint 1.18.1: interface gráfica compilada, sem navegador, React, Node ou Electron.
- Ropey: estrutura de armazenamento de texto em rope, usando índices de caracteres Unicode.
- rfd: diálogos de arquivos do sistema no protótipo desktop.

Slint foi escolhido porque documenta suporte a Windows, Linux, macOS, Android e iOS. Android e iOS **ainda não estão empacotados nem validados neste repositório**. Os diálogos desktop precisarão de adaptadores de arquivos próprios nos sistemas móveis.

## Importante: o editor é provisório
O componente TextEdit do Slint mantém o texto integralmente em uma propriedade string. No protótipo, evitamos recriar Ropey em cada tecla e só atualizamos o modelo de rope nas operações de arquivo.

Isso é suficiente para começar a interface, mas NÃO é um editor incremental/virtualizado nem comprova alta performance com arquivos grandes.

Antes de afirmar que NotionPad Rust supera qualquer editor, implementar e testar:
1. Buffer por edições incrementais (rope ou peça/rope) e histórico undo/redo.
2. Renderização apenas do trecho visível (virtualização).
3. Syntax highlighting incremental com Tree-sitter, sem reprocessar o arquivo inteiro.
4. Renderização de rich text com estilos persistidos separadamente do formato TXT.
5. Busca incremental e contagem de caracteres/linhas sem varredura total por tecla.
6. Capa e emojis carregados sem bloquear digitação; cache com limite de memória.
7. Modos de interface para toque e telas pequenas (Android/iOS).

## Formato dos documentos
- `.txt`, `.md`, `.rs`, `.html` e outros arquivos de código permanecem texto puro.
- Metadados (capa, ícone, preferência visual) ficam em storage separado.
- Rich text precisará de formato próprio (p. ex. `.notionpad`) ou serialização Markdown com limites explícitos.
- Nunca misturar metadados proprietários nos arquivos de código do usuário sem consentimento.

## Critérios de performance (metas a medir, não resultados)
- RAM em repouso (aberto sem arquivo).
- Cold start e warm start.
- Latência p50/p95/p99 de digitação.
- Tempo para carregar 1 MB, 10 MB e 100 MB.
- Latência de rolagem, busca e realce de sintaxe.
- Binário por arquitetura e memória da GPU.
- Comparação com a versão C# e com o Notepad nas mesmas máquinas.

## Portabilidade
- Desktop: `cargo run` inicialmente. Empacotamento e assinatura depois.
- Android: entrada de app Slint/Rust com Android SDK/NDK e empacotamento APK/AAB.
- iOS: target Rust para iOS + Xcode em macOS; empacotamento e assinatura próprios.
- A lógica central fica independente dos frameworks de interface.

## Licenciamento
O código do NotionPad está sob MIT; Slint é uma dependência com licenças próprias.
Para usar a licença royalty-free do Slint é necessário atender à atribuição exigida no app ou no site público de download. Avaliar outras condições antes de distribuir.
