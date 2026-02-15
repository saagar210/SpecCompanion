# Changelog

All notable changes to SpecCompanion will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive CI/CD pipeline with GitHub Actions
  - Multi-platform testing (Linux, macOS, Windows)
  - Automated security scanning (CodeQL, Semgrep, Trivy)
  - Dependency updates via Dependabot
- Cross-platform release builds
  - macOS: DMG and .app bundles (Intel + Apple Silicon)
  - Windows: MSI and NSIS installers
  - Linux: AppImage and Debian packages
- Enhanced spec parser with heading hierarchy tracking
- Requirement marker detection (REQ-*, US-*, FR-*, NFR-*)
- Markdown table support for structured requirements
- Rich HTML export for alignment reports
  - Modern gradient styling
  - Summary cards with coverage metrics
  - Mismatch breakdown with percentages
- LLM integration hardening
  - Model fallback mechanism (Claude Sonnet 4 → 3.5 variants)
  - Improved prompts with few-shot examples
  - Graceful degradation from LLM to template mode
- Comprehensive test suite
  - 40 codebase scanner tests (8 languages)
  - 18 alignment service tests
  - 15 React hook tests with Jest

### Changed
- Enhanced CSV export with metadata header
- Improved error resilience in test generation
- API timeout increased from 60s to 90s
- LLM generation now tracks actual mode used (may differ from requested)

### Fixed
- Path traversal vulnerability protection in file save operations
- GTK dependency issues documented with installation instructions
- Jest ES module configuration resolved

### Security
- Path validation ensures files are written within home directory only
- API keys stored securely in app-specific config directories
- No telemetry or external data transmission (except explicit LLM API calls)
- Automated security scanning in CI pipeline

## [0.1.0] - 2025-XX-XX

### Added
- Initial release of SpecCompanion
- Desktop application built with Tauri v2 + React 19
- SQLite-based local-first architecture
- Markdown spec file parsing with pulldown-cmark
- Multi-language codebase scanning (JavaScript/TypeScript, Python, Go, Java, Ruby, C#, Rust, PHP)
- Requirement-to-code alignment analysis
- Template-based test generation for Jest and pytest
- LLM-powered test generation using Claude API
- Alignment report generation with mismatch detection
- Export reports in JSON, CSV, and HTML formats
- Settings management for API keys and preferences
- Cross-platform support (macOS, Windows, Linux)

### Features

#### Spec Management
- Import markdown specification files
- Automatic requirement extraction
- Section and priority classification
- Requirement type detection (Functional, Non-Functional, UI/UX, Performance, Security)

#### Code Analysis
- Recursive codebase scanning
- Symbol extraction (classes, functions, methods)
- Multi-language support with pattern-based parsing
- Configurable scan exclusions (node_modules, dist, etc.)

#### Test Generation
- Template mode: Rule-based test scaffolding
- LLM mode: AI-powered intelligent test generation
- Framework support: Jest (JavaScript/TypeScript), pytest (Python)
- Context-aware generation using codebase symbols
- Traceability links from tests to requirements

#### Alignment Reporting
- Requirement coverage calculation
- Mismatch classification (no_test_generated, test_failing, not_implemented, partial_coverage)
- Coverage percentage metrics
- Exportable reports (JSON, CSV, HTML)

[Unreleased]: https://github.com/saagar210/SpecCompanion/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/saagar210/SpecCompanion/releases/tag/v0.1.0
