# cofj

A command line journal app built in Rust that allows you to record the various coffee beverages you've consumed.

## Features

- Interactive CLI prompts for coffee details
- Arrow-key navigation for selections
- Automatic plaintext journal generation
- Newest entries appear first
- Stores journal in `cofj.txt` in your current directory

## Installation

### Build from source

```bash
cargo build --release
```

The compiled binary will be located at `./target/release/cofj`

### Optional: Install to PATH

```bash
cargo install --path .
```

This installs the binary to `~/.cargo/bin/cofj` (make sure `~/.cargo/bin` is in your PATH)

## Usage

Run the application:

```bash
./target/release/cofj
```

Or if you installed it:

```bash
cofj
```

The application will prompt you for:

1. **Brand Name** - Text entry (e.g., "Blue Bottle", "Intelligentsia")
2. **Coffee Name** - Text entry (e.g., "Ethiopia Yirgacheffe", "Black Cat")
3. **Origin** - Text entry (e.g., "Ethiopia", "Colombia")
4. **Processing Method** - Select from:
   - Anerobic Fermentation
   - Natural
   - Pulped Natural
   - Washed
5. **Roast** - Select from:
   - Dark
   - Light
   - Medium
6. **Preparation** - Select from:
   - Americano
   - Cortado
   - Drip
   - Espresso
   - Latte
   - Pourover
7. **Rating** - Select from 1-10

After completing all prompts, the entry is saved to `cofj.txt` in the directory where you ran the command.

## Journal Format

Entries are saved in compact plaintext with the following structure:

```
2025-01-15 14:30:00
Brand: Blue Bottle
Name: Ethiopia Yirgacheffe
Origin: Ethiopia
Processing: Washed
Roast: Light
Preparation: Pourover
Rating: 8/10
---
```

Newer entries appear at the top of the file.

## Development

### Running Tests

```bash
cargo test
```

### Project Structure

- `src/entry.rs` - Core data structures and markdown formatting
- `src/lib.rs` - File operations and testable logic
- `src/main.rs` - Interactive CLI interface

## Adding New Options

To add new preparation methods, roasts, or processing methods:

1. Add the variant to the appropriate enum in `src/entry.rs`
2. Update the `Display` implementation for the enum
3. Add the option to the selection list in `src/main.rs`
4. Update the match statement in `src/main.rs`

## License

See LICENSE file for details.
