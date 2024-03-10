# 🌿Olive

A Rust-based compiler for the Olive programming language, which offers a blend of simplicity and intuitive syntax inspired by languages like C, Java, and Python.

## Table of Contents

### 0. [Olive (Entry Point)](compiler/olive)

### 1. [Sample Programs](programs)

### 2. [Language Specification](specification)

### 3. [Symbol Table](compiler/hash-map)

### 4. [Scanner](compiler/scanner)

### 5. [Deterministic Finite Automata](compiler/automata)

### 6. [LL(1) Parser](compiler/parser)

### 7. [Flex](compiler/flex)

### 8. [Bison](compiler/bison)

## Language Features

Olive is fundamentally built upon the following lexical specification:

### Alphabet

- Uppercase (A-Z) and lowercase (a-z) letters of the English alphabet
- Decimal digits (0-9)
- ASCII symbols (32-126)

### Lexic

#### Special Symbols

- Arithmetic operators: `+`, `-`, `*`, `/`, `%`
- Assignment operator: `=`
- Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical operators: `&&`, `||`
- Separators: whitespace, `(`, `)`, `[`, `]`, `{`, `}`, `<`, `>`, `:`, `;`
- Comments: `--`
- Character constant: `'`
- String constant: `"`
- Reserved words: `number`, `char`, `string`, `array`, `input`, `output`, `if`, `else`, `while`

#### Identifiers

- Formed by any combination of uppercase and lowercase letters.

#### Constants

- **Integer:** Formed by an optional sign (`+` or `-`), followed by digits. Example: `123`, `-456`
- **Character:** Enclosed in single quotes. Example: `'a'`
- **String:** Enclosed in double quotes. Example: `"Hello, World!"`

## Key Features of Olive

- **Variable Declarations and Types:** Like C and Java, Olive supports explicit variable declarations with basic types (number, char, string) and arrays.

- **Control Flow:** The language incorporates control flow constructs similar to those in C-like languages, including if, else, and while loops.

- **Expressions and Operators:** Olive's expression syntax and operators (arithmetic, logical) are reminiscent of languages like Python and JavaScript, allowing for familiar arithmetic and logical operations.

- **Input/Output Operations:** The input and output statements in Olive are straightforward, much like the I/O operations in Python and Java.

- **Comments:** Olive uses -- for single-line comments, a style common in SQL and Lua.

### Similarities to Other Languages

- **C/Java-like Syntax:** Olive's variable declarations, data types, and control structures have a syntax and behavior similar to C and Java.

- **Python/JavaScript Expressiveness:** The ease of writing expressions and the simplified I/O operations in Olive are akin to scripting languages like Python and JavaScript.

- **Structured Programming:** Olive encourages structured programming practices, making it easy for developers familiar with traditional programming paradigms to adapt.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

### Installation

1. Clone the repository.

2. Navigate to the `compiler/olive` directory.

3. Build the project using Cargo:

```bash
cargo build
```

### Usage

To compile an Olive program, run the following command:

```bash
cargo run <path-to-olive-file>
```

For example:

```bash
cargo run programs/p1.oli
```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](./LICENSE) file for details.
