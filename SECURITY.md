# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities by emailing:

**security@speccompanion.dev** (or create a private security advisory on GitHub)

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information (as much as you can provide) to help us better understand the nature and scope of the possible issue:

- Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

## Disclosure Policy

When the security team receives a security bug report, they will:

1. Confirm the problem and determine the affected versions
2. Audit code to find any similar problems
3. Prepare fixes for all supported releases
4. Release new security fix versions as soon as possible

## Security Update Process

Security updates will be released with the following process:

1. A security advisory will be published on GitHub
2. Patched versions will be released for all supported versions
3. The vulnerability will be disclosed publicly after users have had time to update

## Automated Security Scanning

This project uses the following automated security tools:

- **Dependabot**: Automatic dependency updates and security alerts
- **CodeQL**: Semantic code analysis for security vulnerabilities
- **Semgrep**: Static analysis for security patterns
- **Trivy**: Vulnerability scanning for dependencies
- **cargo-audit**: Rust dependency security audit
- **npm audit**: NPM dependency security audit

## Security Best Practices

When contributing to SpecCompanion, please follow these security guidelines:

### Input Validation
- Always validate and sanitize user inputs
- Use parameterized queries for database operations (already enforced via rusqlite)
- Validate file paths before filesystem operations

### Authentication & Authorization
- Never commit API keys or secrets to the repository
- Use environment variables or secure configuration for sensitive data
- Validate user permissions before destructive operations

### Data Protection
- Sensitive data (API keys) are stored in app-specific directories with appropriate permissions
- Database files are stored locally with user-level permissions
- No telemetry or analytics without explicit user consent

### Code Execution
- All shell commands must be properly escaped
- Avoid `eval()` or dynamic code execution
- Validate inputs to subprocess calls

### Dependencies
- Keep dependencies up to date
- Review dependency changes in Dependabot PRs
- Audit new dependencies before adding them
- Use `cargo audit` and `npm audit` before releases

## Known Security Considerations

### Local-First Architecture
SpecCompanion is designed as a local-first application:
- All data is stored locally on the user's machine
- No data is transmitted to external servers (except LLM API calls when explicitly configured)
- Users are responsible for their own data backups

### LLM API Integration
When using LLM features:
- API keys are stored in app-specific config directories
- Requirement text may be sent to Claude API (Anthropic)
- Users should review their organization's policies before using LLM features with proprietary code

### File System Access
The application has access to:
- User's home directory for codebase scanning
- App-specific data directory for database and settings
- User-selected directories for spec files and test output

Path traversal protections are implemented in `src-tauri/src/commands/test_gen.rs:162-183`.

## Contact

For general security questions or concerns, please open a GitHub Discussion or contact the maintainers.
