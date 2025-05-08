# CLI Calculator

A simple command-line calculator built with Rust that performs basic arithmetic operations.

## Features

- ✨ Interactive command-line interface
- 🔢 Supports basic arithmetic operations:
  - Addition (+)
  - Subtraction (-)
  - Multiplication (*)
  - Division (/)
- 🛡️ Input validation and error handling
- 🔄 Continuous operation mode

## Usage

1. Start the program:
```bash
cargo run
```

2. Follow the interactive prompts:
   - Enter the first number
   - Enter the second number
   - Choose an operation (+, -, *, /)
   - View the result
   - Choose to continue or exit

## Example

```
════════════════════════════════
✨ Welcome to the CLI Calculator ✨
════════════════════════════════

Enter the first number
10
Enter the second number
5
Enter the operation (+, -, *, /)
+
Result: 10 + 5 = 15

Do you want to continue? (y/n)
```

## Error Handling

- Handles invalid number inputs
- Prevents division by zero
- Validates operation input

## Building from Source

```bash
git clone <repository-url>
cd cli-calc
cargo build
cargo run
```

## Requirements

- Rust (Latest stable version)
- Cargo (Rust's package manager)