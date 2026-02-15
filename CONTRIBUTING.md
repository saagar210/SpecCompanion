# Contributing to SpecCompanion

Thank you for your interest in contributing to SpecCompanion! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## Getting Started

### Development Environment

1. **Prerequisites**
   ```bash
   # Install Node.js (LTS) and pnpm
   curl -fsSL https://get.pnpm.io/install.sh | sh

   # Install Rust (stable)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install Tauri CLI
   cargo install tauri-cli --version "^2"
   ```

2. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/SpecCompanion.git
   cd SpecCompanion
   ```

3. **Install Dependencies**
   ```bash
   pnpm install
   ```

4. **Install System Dependencies** (Linux only)
   ```bash
   sudo apt-get install -y \
     libgtk-3-dev \
     libwebkit2gtk-4.0-dev \
     libpango1.0-dev \
     libatk1.0-dev \
     libgdk-pixbuf2.0-dev \
     libjavascriptcoregtk-4.0-dev \
     libsoup2.4-dev \
     librsvg2-dev
   ```

5. **Run Development Server**
   ```bash
   pnpm tauri dev
   ```

### Project Structure

```
SpecCompanion/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── hooks/              # Custom React hooks
│   ├── lib/                # Utilities and helpers
│   └── __tests__/          # Frontend tests
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri command handlers
│   │   ├── db/             # Database queries and schema
│   │   ├── models/         # Data structures
│   │   └── services/       # Business logic
│   └── Cargo.toml
├── .github/workflows/      # CI/CD pipelines
└── package.json
```

## Development Workflow

### Branch Naming Convention

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring
- `test/description` - Test additions or improvements

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, no logic change)
- `refactor`: Code restructuring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(scanner): add PHP language support
fix(alignment): handle edge case with missing test results
docs(readme): update installation instructions
test(parser): add tests for table parsing
```

### Running Tests

**Rust Backend Tests:**
```bash
cd src-tauri
cargo test --lib
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

**Frontend Tests:**
```bash
npm test
npm run lint
npx tsc --noEmit
```

### Code Quality

- **Rust**: Follow Rust API guidelines and use `clippy` recommendations
- **TypeScript**: Strict mode is enabled; no `any` types without justification
- **Formatting**: Run `cargo fmt` and `prettier` before committing
- **Coverage**: Aim for >80% test coverage on new code

## Pull Request Process

1. **Create a Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Your Changes**
   - Write clean, documented code
   - Add tests for new functionality
   - Update documentation as needed

3. **Test Thoroughly**
   ```bash
   # Run all tests
   cd src-tauri && cargo test --lib
   cd .. && npm test

   # Check linting
   cd src-tauri && cargo clippy --all-targets
   cd .. && npm run lint
   ```

4. **Commit Your Changes**
   ```bash
   git add .
   git commit -m "feat(scope): description of changes"
   ```

5. **Push to Your Fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Open a Pull Request**
   - Use a clear, descriptive title
   - Reference any related issues
   - Describe what changed and why
   - Include screenshots for UI changes
   - Ensure CI checks pass

### PR Review Checklist

Before requesting review, ensure:

- [ ] All tests pass locally
- [ ] Code follows project style guidelines
- [ ] New code has test coverage
- [ ] Documentation is updated
- [ ] No console warnings or errors
- [ ] Commit messages follow convention
- [ ] PR description is clear and complete

## Areas for Contribution

### High Priority
- Additional language support in codebase scanner (PHP, Kotlin, Swift)
- AST-based parsing for more accurate symbol extraction
- VS Code extension integration
- CLI version for CI/CD integration
- Custom test template editor in UI

### Medium Priority
- Additional test frameworks (Mocha, JUnit, RSpec)
- Spec diff visualization (track changes over time)
- Coverage trend charts
- Bulk test execution improvements
- Test result caching and incremental runs

### Good First Issues
- Documentation improvements
- Error message clarity
- UI polish and accessibility
- Additional export formats
- More comprehensive test coverage

## Security Vulnerabilities

**Do not open public issues for security vulnerabilities.**

Please review [SECURITY.md](SECURITY.md) for responsible disclosure process.

## Questions?

- Open a [GitHub Discussion](https://github.com/saagar210/SpecCompanion/discussions) for questions
- Check existing issues and PRs before creating new ones
- Join our community chat (link TBD)

## License

By contributing to SpecCompanion, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to SpecCompanion! 🚀
