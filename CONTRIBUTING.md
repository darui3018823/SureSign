# Contributing to SureSign

Thank you for your interest in contributing to SureSign! We welcome contributions from the community. This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Please note that this project is governed by our [Code of Conduct](./Code_of_Conduct.md). By participating in this project, you agree to abide by its terms.

## How to Contribute

### Reporting Bugs

If you encounter a bug, please create an issue with the following information:

- **Description**: A clear and concise description of the bug
- **Steps to Reproduce**: Steps to reproduce the behavior
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**: Your OS, Rust version, and any other relevant details
- **Screenshots/Logs**: If applicable, include error messages or logs

### Suggesting Enhancements

We welcome suggestions for new features or improvements:

- **Description**: A clear and concise description of the enhancement
- **Motivation**: Why you believe this enhancement would be useful
- **Alternative Solutions**: Any alternative approaches you've considered
- **Additional Context**: Any other relevant information

### Submitting Pull Requests

1. **Fork the Repository**: Click the "Fork" button on GitHub

2. **Clone Your Fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/SureSign.git
   cd SureSign
   ```

3. **Create a Feature Branch**:
   ```bash
   git checkout -b feat/your-feature-name
   ```
   or for bug fixes:
   ```bash
   git checkout -b fix/your-bug-fix
   ```

4. **Make Your Changes**: Implement your feature or bug fix

5. **Follow Commit Message Conventions**:
   We follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat: add new feature`
   - `fix: fix a bug`
   - `docs: update documentation`
   - `refactor: refactor code`
   - `test: add tests`
   - `chore: update dependencies`

6. **Test Your Changes**:
   ```bash
   cargo test
   cargo build --release
   ```

7. **Push to Your Fork**:
   ```bash
   git push origin feat/your-feature-name
   ```

8. **Create a Pull Request**: Click "New Pull Request" on GitHub and provide:
   - A clear title describing your changes
   - A detailed description of what you changed and why
   - References to related issues (if any)

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git

### Building from Source

```bash
git clone https://github.com/darui3018823/SureSign.git
cd SureSign
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

## Project Structure

```
SureSign/
├── src/
│   ├── main.rs           # Application entry point
│   ├── cert.rs           # Certificate generation and handling
│   ├── cli.rs            # Command-line interface
│   ├── i18n.rs           # Internationalization
│   └── interactive.rs    # Interactive mode
├── scripts/              # Build and utility scripts
├── Cargo.toml            # Project manifest
└── README.md             # Project documentation
```

## Code Style

- Follow Rust conventions and best practices
- Use `cargo fmt` to format your code
- Use `cargo clippy` to check for common mistakes

```bash
cargo fmt
cargo clippy --all-targets
```

## Pull Request Review Process

1. Your PR will be reviewed by project maintainers
2. We may request changes or ask clarifying questions
3. Once approved, your PR will be merged
4. Your contribution will be credited in the next release

## Release Process

We follow [Semantic Versioning](https://semver.org/):
- MAJOR version for incompatible API changes
- MINOR version for new backwards-compatible functionality
- PATCH version for backwards-compatible bug fixes

## Questions?

Feel free to:
- Open an issue with the label `question`
- Contact us at [contact@daruks.com](mailto:contact@daruks.com)

Thank you for contributing to SureSign!
