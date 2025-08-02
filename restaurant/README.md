# Restaurant - Exemplo de Módulos em Arquivos Separados

Este projeto demonstra os conceitos apresentados no [Capítulo 7.5 do livro do Rust](https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html) sobre como separar módulos em diferentes arquivos.

## Estrutura do Projeto

```
src/
├── lib.rs                          # Raiz da crate (biblioteca)
├── main.rs                         # Executável para testar a biblioteca
├── front_of_house.rs              # Módulo front_of_house
├── front_of_house/
│   ├── hosting.rs                 # Submódulo hosting (estilo moderno)
│   └── serving/
│       └── mod.rs                 # Submódulo serving (estilo antigo)
```

## Evolução do Código

### 1. Versão Inicial (tudo em um arquivo)
Inicialmente, todos os módulos estavam definidos dentro de `lib.rs`:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { ... }
    }
    // ...
}
```

### 2. Separando o módulo principal
O módulo `front_of_house` foi extraído para seu próprio arquivo:

**lib.rs:**
```rust
mod front_of_house;  // Declaração do módulo
```

**front_of_house.rs:**
```rust
pub mod hosting {
    pub fn add_to_waitlist() { ... }
}
```

### 3. Separando submódulos
Os submódulos foram extraídos para arquivos/diretórios próprios:

- `hosting` → `front_of_house/hosting.rs` (estilo moderno)
- `serving` → `front_of_house/serving/mod.rs` (estilo antigo)

## Estilos de Organização de Arquivos

O Rust suporta dois estilos para organizar módulos:

### Estilo Moderno (Recomendado)
- Módulo `foo` → arquivo `foo.rs`
- Submódulo `foo::bar` → arquivo `foo/bar.rs`

### Estilo Antigo (Ainda Suportado)
- Módulo `foo` → arquivo `foo/mod.rs`
- Submódulo `foo::bar` → arquivo `foo/bar/mod.rs`

## Conceitos Importantes

1. **Declaração vs Definição**: `mod nome;` declara o módulo, o compilador procura o arquivo correspondente
2. **Hierarquia de Diretórios**: A estrutura de pastas reflete a árvore de módulos
3. **Visibilidade**: Módulos são privados por padrão, use `pub mod` para torná-los públicos
4. **Importação com `use`**: Traz itens para o escopo atual para facilitar o uso

## Como Executar

```bash
# Compilar e executar
cargo run

# Apenas compilar
cargo build

# Executar testes (se houver)
cargo test
```

## Pontos de Aprendizado

- A separação de módulos é puramente organizacional - não afeta a funcionalidade
- O compilador Rust entende a estrutura de módulos através das declarações `mod`
- Você pode misturar os dois estilos no mesmo projeto (mas não é recomendado)
- A estrutura de arquivos deve espelhar a hierarquia de módulos