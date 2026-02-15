# Spec Companion

[![CI](https://github.com/saagar210/SpecCompanion/workflows/CI/badge.svg)](https://github.com/saagar210/SpecCompanion/actions/workflows/ci.yml)
[![Security Scanning](https://github.com/saagar210/SpecCompanion/workflows/Security%20Scanning/badge.svg)](https://github.com/saagar210/SpecCompanion/actions/workflows/security.yml)
[![Release](https://github.com/saagar210/SpecCompanion/workflows/Release/badge.svg)](https://github.com/saagar210/SpecCompanion/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A desktop app that verifies your code actually implements what your spec says it should. Upload a markdown spec, generate tests from requirements, run them against your codebase, and get alignment reports showing exactly where coverage gaps exist.

Built with Tauri v2 (Rust backend, React frontend). Runs locally, works offline, keeps your code and specs private.

## What It Does

**Spec in, coverage report out.** The full workflow:

1. **Point it at your project** -- select your codebase directory, upload a markdown spec
2. **Requirements get extracted automatically** -- the parser identifies functional requirements, constraints, and user stories from your spec's structure
3. **Generate tests from requirements** -- choose template mode (instant, offline) or LLM mode (Claude API, richer tests) for Jest or PyTest
4. **Execute tests against your codebase** -- runs Jest/PyTest with real-time progress, captures stdout/stderr, enforces timeouts
5. **Get an alignment report** -- see coverage percentage, mismatch breakdown, and exactly which requirements lack tests, have failing tests, or are only partially covered
6. **Export and share** -- JSON, HTML, or CSV reports

## Key Strengths

### Requirement Extraction That Works
The Markdown parser uses `pulldown-cmark` to walk the AST, not regex on raw text. It understands heading hierarchy, identifies requirement-bearing sections (Requirements, Features, Acceptance Criteria, User Stories, Constraints), and classifies each requirement by type (functional, non-functional, constraint) and priority. Re-parse anytime the spec changes.

### Two Test Generation Modes
- **Template mode** -- instant, offline, zero config. Produces Jest `describe/it` or PyTest `class/def test_` skeletons with Arrange/Act/Assert structure, traceability comments linking back to requirements, and relevant import suggestions based on codebase symbol matching.
- **LLM mode** -- sends requirement context + your codebase's function/class signatures to Claude, gets back tests with meaningful assertions, edge cases, and realistic mock data. Requires an API key (set once in Settings).

### Codebase-Aware
Before generating tests, the app scans your project for code symbols (functions, classes, methods) across TypeScript, JavaScript, Python, Rust, Go, Java, Ruby, and C#. These symbols provide context for both template and LLM generation, so generated tests reference your actual code.

### Real Test Execution
Not a mock runner. Spawns actual `npx jest` or `python -m pytest` processes against your codebase directory. 120-second timeout per test prevents runaways. Stdout/stderr captured in separate threads to avoid pipe deadlocks. Results stored with execution time for trend analysis.

### Alignment Analysis
The report engine walks every requirement and checks: does a test exist? Has it been executed? Did it pass? The result is a coverage percentage and a categorized mismatch list:
- **No Test Generated** -- requirement has no test at all
- **Not Implemented** -- test exists but was never run
- **Test Failing** -- all tests for this requirement fail
- **Partial Coverage** -- some tests pass, some fail

### Data Stays Local
SQLite database stored in your app data directory. No cloud sync, no telemetry. The only network call is the optional Claude API for LLM test generation, and only when you explicitly trigger it.

## Architecture

```
Frontend (React 19 + TypeScript + Tailwind v4)
    |
    | Tauri IPC (invoke)
    |
Backend (Rust + Tokio async runtime)
    |
    |-- SQLite (rusqlite, 7 tables, WAL mode)
    |-- Spec Parser (pulldown-cmark AST)
    |-- Codebase Scanner (multi-language symbol extraction)
    |-- Template Generator (pattern-matched test skeletons)
    |-- LLM Generator (Claude API via reqwest)
    |-- Test Runner (process spawn with timeout)
    |-- Alignment Engine (requirement-to-result analysis)
    |-- Git Service (libgit2 for branch/commit/diff info)
```

**Frontend state**: TanStack Query v5 handles caching, invalidation, and loading states. No Redux.

**Database schema**: `projects` > `specs` > `requirements` > `generated_tests` > `test_results`, plus `alignment_reports` > `alignment_mismatches`. All with UUID primary keys, RFC3339 timestamps, and cascading deletes.

**Security**: All file operations validate paths are within the home directory. File size limits (50MB for spec uploads, 1MB for scanned source files). Parameterized SQL queries. Filename sanitization on uploads. Directory traversal depth limits. Codebase paths canonicalized on storage.

## Supported Frameworks

| Test Framework | Command | Language |
|---------------|---------|----------|
| Jest | `npx jest` | JavaScript / TypeScript |
| PyTest | `python -m pytest` | Python |

Codebase scanning supports: TypeScript, JavaScript, Python, Rust, Go, Java, Ruby, C#.

Spec format: Markdown (`.md`, `.txt`, `.markdown`).

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (LTS)
- [pnpm](https://pnpm.io/)
- [Rust](https://rustup.rs/) (stable)
- Tauri v2 CLI: `cargo install tauri-cli --version "^2"`

### Development

```bash
pnpm install
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

Produces `.app` + `.dmg` on macOS, `.msi` on Windows, `.deb`/`.AppImage` on Linux.

### Configuration

Open **Settings** in the app to configure:
- **Claude API Key** -- required only for LLM test generation mode
- **Default Framework** -- Jest or PyTest
- **Default Generation Mode** -- Template or LLM
- **Scan Exclusion Patterns** -- directories to skip during codebase scanning (e.g., `dist, build, .cache`)

## Export Formats

| Format | Contents |
|--------|----------|
| JSON | Full report object with all mismatches and metadata |
| HTML | Styled table with coverage stats, color-coded mismatch badges |
| CSV | Tabular data: requirement ID, section, mismatch type, details |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop framework | Tauri v2 |
| Backend | Rust (2021 edition) |
| Async runtime | Tokio |
| Database | SQLite via rusqlite |
| HTTP client | reqwest |
| Git | libgit2 via git2 |
| Markdown parsing | pulldown-cmark |
| Frontend | React 19 + TypeScript (strict) |
| Styling | Tailwind CSS v4 |
| State management | TanStack Query v5 |
| Charts | Recharts |
| Code highlighting | prism-react-renderer |
| Markdown rendering | react-markdown |

## CI/CD & Releases

This project uses GitHub Actions for continuous integration, security scanning, and automated releases:

- **CI Pipeline** (`.github/workflows/ci.yml`)
  - Runs on every push and PR to main/develop
  - Multi-platform testing (Linux, macOS, Windows)
  - Rust tests, Clippy linting, and formatting checks
  - Frontend TypeScript type checking, ESLint, and Jest tests
  - Build verification on all platforms

- **Security Scanning** (`.github/workflows/security.yml`)
  - Weekly scheduled scans + on-demand
  - Cargo audit for Rust dependencies
  - NPM audit for frontend dependencies
  - CodeQL analysis for security vulnerabilities
  - Semgrep SAST scanning
  - Trivy filesystem vulnerability scanning
  - Dependency review on PRs

- **Release Automation** (`.github/workflows/release.yml`)
  - Triggered by version tags (`v*.*.*`) or manual dispatch
  - Cross-platform builds:
    - macOS: `.dmg` (Apple Silicon + Intel)
    - Windows: `.msi` + `.exe` (NSIS installer)
    - Linux: `.deb` + `.AppImage`
  - Automatic GitHub Release creation with installers
  - Optional code signing (configure secrets for production)

- **Dependency Management** (`.github/dependabot.yml`)
  - Automated weekly dependency updates
  - Separate groups for NPM, Cargo, and GitHub Actions
  - Security vulnerability alerts

### Creating a Release

1. Update version in `src-tauri/Cargo.toml` and `package.json`
2. Update `CHANGELOG.md` with release notes
3. Create and push a version tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
4. GitHub Actions will automatically build and create a draft release
5. Review the draft release and publish when ready

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

For security vulnerabilities, please see [SECURITY.md](SECURITY.md).

## License

MIT
