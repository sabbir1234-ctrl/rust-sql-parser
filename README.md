# Rust SQL Parser

An interactive command-line **SQL parser written in Rust**. Type a query and it is tokenized, parsed and printed as an abstract syntax tree (AST).

![Rust](https://img.shields.io/badge/Rust-2024%20edition-000000?logo=rust) ![No dependencies](https://img.shields.io/badge/dependencies-none-brightgreen)

## Supported SQL

**`SELECT`** with column lists, `FROM`, `WHERE` and `ORDER BY … ASC | DESC`

```sql
SELECT name, age FROM users WHERE age >= 18 AND NOT banned ORDER BY age DESC
```

**`CREATE TABLE`** with `INT`, `BOOL` and `VARCHAR` columns and constraints (`PRIMARY KEY`, `NOT NULL`, `NULL`, `CHECK (…)`)

```sql
CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR NOT NULL, age INT CHECK (age > 0))
```

**Expressions:** identifiers, numbers, strings, `TRUE` / `FALSE` / `NULL`, unary `NOT` and `-`, and the binary operators `= != < <= > >= + - * / AND OR`.

## How it works

```
input ──▶ Tokenizer ──▶ Parser ──▶ Statement (AST) ──▶ pretty-printed with {:#?}
```

| File | Role |
|------|------|
| `src/token.rs` | `Token` and `Keyword` enums |
| `src/tokenizer.rs` | Turns the input string into tokens |
| `src/parser.rs` | Recursive-descent parser; expressions use **precedence climbing** (`OR` < `AND` < comparison < `+ -` < `* /`) |
| `src/statement.rs` | AST types: `SelectStatement`, `CreateTableStatement`, `Expression`, `BinaryOperator`, … |
| `src/main.rs` | Interactive REPL loop (`exit` to quit) |

## Run

```bash
cargo run
```

```
Welcome to the SQL parser CLI!
Enter your SQL query: SELECT id FROM users WHERE id > 5
Parsed SQL statement:
Select(
    SelectStatement {
        columns: ["id"],
        table: "users",
        where_clause: Some(BinaryOp { left: Identifier("id"), op: Gt, right: Literal(Number(5)) }),
        order_by: None,
    },
)
```
