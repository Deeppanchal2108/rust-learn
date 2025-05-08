# Mini CLI Program

A command-line interface (CLI) program built with Rust that performs basic operations like greetings and arithmetic calculations.

## Features

- 👋 Greeting functionality
- 🔢 Basic arithmetic operations:
  - Addition
  - Subtraction
  - Multiplication
  - Division
- ⚡ Command-line argument processing
- 🛡️ Input validation

## Usage

Run the program using `cargo run` followed by a command and its arguments:

```bash
cargo run -- <command> <args...>
```

### Available Commands

1. **Greeting**
```bash
cargo run -- greet <name>
```

2. **Addition**
```bash
cargo run -- add <number1> <number2>
```

3. **Subtraction**
```bash
cargo run -- sub <number1> <number2>
```

4. **Multiplication**
```bash
cargo run -- mul <number1> <number2>
```

5. **Division**
```bash
cargo run -- div <number1> <number2>
```

## Examples

```bash
# Greeting
cargo run -- greet John
> Hello John

# Addition
cargo run -- add 5 3
> The sum of 5 and 3 is 8

# Multiplication
cargo run -- mul 4 6
> The product of 4 and 6 is 24
```

## Error Handling

The program includes error handling for:
- Missing command-line arguments
- Invalid number inputs
- Unknown commands

## Building from Source

```bash
git clone <repository-url>
cd mini-cli
cargo build
cargo run -- <command> <args...>
```

## Requirements

- Rust (Latest stable version)
- Cargo (Rust's package manager)