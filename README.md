# Bunka

`bunka` is a lightweight Rust command-line utility that approximates any decimal (floating-point number) into its fractional representation (numerator/denominator) using the **Continued Fraction Expansion** algorithm.

## Features

- **High Precision**: Custom tolerance and maximum denominator limits allow for accurate fractional approximations.
- **Ultra-lightweight**: Highly optimized release binary size (~138.5 KB) with zero external dependencies for the CLI version.
- **Mathematical Integrity**: Utilizes the continued fraction algorithm to find the best rational approximation.
- **Feature-gated GUI**: Supports an ultra-minimalistic, frameless, transparent GUI overlay using `eframe`/`egui` (enabled via the `gui` feature).

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (edition 2024 supported)

### Installation & Build

#### CLI Version (Default)
Clone this repository and build the release binary:

```bash
cargo build --release
```

The optimized binary will be generated at `target/release/bunka.exe` (or `target/release/bunka` on Linux/macOS).

#### GUI Version
To build and run the GUI version:

```bash
cargo run --release --features gui
```

## Usage

Run the program by passing a decimal number as a command-line argument:

```bash
bunka <decimal_number> [options]
```

### Options

- `-d, --max-den <value>`: Maximum denominator allowed for approximation (default: 100,000)
- `-t, --tolerance <value>`: Tolerance criteria for the calculation (default: 1e-6)
- `-h, --help`: Prints help information
- `-v, -V, --version`: Prints version information

### Examples

```bash
$ bunka 0.142857
1/7

# Set maximum denominator to 100
$ bunka 3.14159265 -d 100
22/7

# Increase precision criteria
$ bunka 0.142857 -t 1e-10 -d 10000000
142857/1000000
```

### Errors

If the argument is missing or not a valid floating-point number, `bunka` will output an error message to `stderr` and exit with code `1`:

```bash
$ bunka
使用方法: bunka <小数点数> [オプション]
例) bunka 0.142857  ->  1/7

$ bunka invalid
エラー: 'invalid' は無効な浮動小数点数です
```

## Documentation

For more detailed documentation, please refer to:
- [System Specification (SPEC.md)](docs/SPEC.md) - Algorithm details, arguments, and GUI plans.
- [System Diagrams (DIAGRAM.md)](docs/DIAGRAM.md) - Flowcharts and architecture.
- [Performance & Footprints (FOOTPRINTS.md)](docs/FOOTPRINTS.md) - Binary size and memory usage statistics.
- [Test Report (TEST_REPORT.md)](docs/TEST_REPORT.md) - Test cases and verification steps.
- [Project Setup Template Guide (project_template_guide.md)](docs/project_template_guide.md) - Standard configurations for editors, CI/CD, and Dependabot.


## Development

This repository includes unified editor configurations and automated workflows:
- **Editor Configurations**: [.editorconfig](.editorconfig) and [.vscode/settings.json](.vscode/settings.json) are provided to ensure consistent code styling.
- **CI/CD**: Automatic testing is run on PRs/pushes via [.github/workflows/ci.yml](.github/workflows/ci.yml). Automatic release binaries (both CLI and GUI versions in a zip file) are created and uploaded to GitHub Releases when pushing a tag (`v*`) via [.github/workflows/release.yml](.github/workflows/release.yml).
- **Dependabot**: Automatically checks for dependency updates weekly ([.github/dependabot.yml](.github/dependabot.yml)).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

