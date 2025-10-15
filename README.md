# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição
Sistema de busca simples implementado em Rust usando uma estrutura de dados `HashMap` (tabela hash) para indexar produtos.  
Permite buscas rápidas por nome, simulando um sistema básico de recomendação e busca.

## Tecnologias Utilizadas
- Rust
- HashMap (estrutura de dados)
- Cargo (gerenciador de pacotes Rust)

## Como Executar
```bash
cargo run
```

## Como Executar os Testes
```bash
cargo test
```

## Estrutura do Projeto
- `src/main.rs` — Código principal do sistema
- `tests/search_tests.rs` — Testes automatizados
- `Cargo.toml` — Configuração do projeto
- `README.md` — Documentação

## Algoritmos e Estruturas de Dados
- Uso de `HashMap` para indexação eficiente de produtos por nome.
- Busca parcial por string (case-insensitive).

## Considerações de Desempenho
A busca é rápida para catálogos pequenos e escalável com técnicas como indexação por prefixos ou árvores B em implementações futuras.
