# Delta Plan

## A) Executive Summary

### Current state (repo-grounded)
- Desktop app with Tauri v2 backend + React/TypeScript frontend, with routes for dashboard, project, spec, generation, execution, reports, settings (`src/App.tsx`, `src/pages/*`).
- Core backend command surface exists for projects/specs/test generation/test execution/reports/git (`src-tauri/src/commands/*`).
- Persistence is SQLite via rusqlite with migration v1 and seven primary tables + indexes (`src-tauri/src/db/schema.rs`).
- Requirement extraction is markdown-AST driven but list-item centric and heuristic (`src-tauri/src/services/spec_parser.rs`).
- Test generation supports template mode and Claude LLM mode (`src-tauri/src/services/template_generator.rs`, `src-tauri/src/services/llm_generator.rs`).
- Execution supports only Jest and PyTest with timeout and stdout/stderr capture (`src-tauri/src/services/test_runner.rs`).
- Alignment report engine classifies requirement coverage/mismatches (`src-tauri/src/services/alignment.rs`).
- Codebase scanner advertises many language extensions but symbol extraction is only implemented for JS/TS, Python, Rust (`src-tauri/src/services/codebase_scanner.rs`).
- Baseline verification currently blocked by dependency fetch 403 for npm/crates (`codex/VERIFICATION.md`).

### Key risks
- Verification cannot run end-to-end due external registry/network restrictions.
- Scanner-language mismatch causes weak context quality for LLM/template generation.
- Heuristic parsing/scanning can create false confidence in alignment metrics.
- Single global DB connection mutex may become throughput bottleneck under heavier async command use.

### Improvement themes (prioritized)
1. Close scanner capability gap for advertised languages (Go, Java, Ruby, C#).
2. Add unit tests around symbol extraction edge cases and new language handlers.
3. Improve auditable session scaffolding + checkpoints for interruption/resume.

## B) Constraints & Invariants (Repo-derived)

### Explicit invariants
- Command contracts and IPC names remain unchanged.
- DB schema remains at v1 (no migration changes in this run).
- Supported execution frameworks remain Jest/PyTest only.
- Security checks around file path/home directory boundaries remain intact.

### Implicit invariants (inferred)
- Scanner should never execute user code; read-only file parsing only.
- Failure in scanner should degrade gracefully and not fail generation path.
- Existing symbol extraction shape (`CodeSymbol {name,kind,file_path}`) remains stable for consumers.

### Non-goals
- No UI redesign/refactor.
- No new persistence schema.
- No provider abstraction rewrite for LLM.
- No CI pipeline introduction in this run.

## C) Proposed Changes by Theme (Prioritized)

### Theme 1: Scanner language coverage parity
- Current approach: extension list includes `go/java/rb/cs`, but extraction no-ops for these extensions.
- Proposed change: add lightweight extraction functions for Go/Java/Ruby/C# declarations.
- Why: improves code-context relevance for generated tests without changing public APIs.
- Tradeoffs: regex/line heuristics still imperfect; AST parsing deferred.
- Scope boundary: only service-level extraction logic + tests.
- Migration approach: additive change behind existing scan flow.

### Theme 2: Scanner regression tests
- Current approach: no tests in scanner.
- Proposed change: add unit tests to verify extraction on representative snippets.
- Why: prevent regressions and support future parser improvements.
- Tradeoffs: tests validate heuristic behavior, not full language grammar.
- Scope boundary: tests in `codebase_scanner.rs` only.

### Theme 3: Session artifacts and checkpoints
- Current approach: no codex interruption-hardening files.
- Proposed change: maintain required artifacts and checkpoint logs.
- Why: satisfy long-running autonomous run discipline and safe resume.
- Tradeoffs: documentation overhead.

## D) File/Module Delta (Exact)

### ADD
- `codex/SESSION_LOG.md` — per-step implementation log.
- `codex/PLAN.md` — execution plan.
- `codex/DECISIONS.md` — judgment calls.
- `codex/CHECKPOINTS.md` — checkpoint snapshots + rehydration blocks.
- `codex/VERIFICATION.md` — baseline/per-step/final verification evidence.
- `codex/CHANGELOG_DRAFT.md` — grouped change summary draft.

### MODIFY
- `src-tauri/src/services/codebase_scanner.rs` — add extraction handlers + unit tests.

### REMOVE/DEPRECATE
- None.

### Boundary rules
- Allowed: changes in scanner service + codex docs.
- Forbidden: command signatures, DB migrations, UI contract changes in this run.

## E) Data Models & API Contracts (Delta)
- Current: `CodeSymbol` struct used as scan output.
- Proposed: no structural change to `CodeSymbol`; only richer population for additional languages.
- Compatibility: backward compatible (same fields/types).
- Migrations: none.
- Versioning strategy: not applicable (internal additive behavior).

## F) Implementation Sequence (Dependency-Explicit)

1. **S1**: Create session artifacts + discovery logs/checkpoint #1.
   - Files: `codex/*`
   - Preconditions: baseline command attempts complete.
   - Verification: `git status --short`; manual review of markdown files.
   - Rollback: remove `codex/*` additions.

2. **S2**: Implement language extractors for go/java/ruby/csharp in scanner.
   - Files: `src-tauri/src/services/codebase_scanner.rs`
   - Preconditions: S1 complete.
   - Verification: `cargo test codebase_scanner` (expected blocked in this environment; still attempt).
   - Rollback: revert scanner file to pre-step version.

3. **S3**: Add scanner unit tests for new handlers + existing behavior.
   - Files: `src-tauri/src/services/codebase_scanner.rs`
   - Preconditions: S2 complete.
   - Verification: `cargo test codebase_scanner` (attempt), static inspection.
   - Rollback: revert test module changes.

4. **S4**: Harden logs, decisions, changelog, final checkpoint, full verification attempt.
   - Files: `codex/SESSION_LOG.md`, `codex/DECISIONS.md`, `codex/CHECKPOINTS.md`, `codex/CHANGELOG_DRAFT.md`, `codex/VERIFICATION.md`
   - Preconditions: S1-S3 complete.
   - Verification: `pnpm build`, `cargo test` (attempts; expected environment blockers documented).
   - Rollback: revert codex docs.

## G) Error Handling & Edge Cases
- Current error patterns: `AppError` enum propagated from commands/services.
- Proposed improvements: none to taxonomy this run; maintain existing propagation.
- Edge cases to cover in scanner tests:
  - Go receiver methods and free functions
  - Java class + method declarations
  - Ruby `def self.x` and `class` detection
  - C# `class` and method signatures

## H) Integration & Testing Strategy
- Integration points: scanner output consumed by generation modes.
- Unit tests: add/expand scanner tests.
- Regression tests: scanner-specific only.
- Definition of Done:
  - scanner supports added languages heuristically,
  - tests authored for handlers,
  - verification attempts and blockers documented,
  - checkpoints complete.

## I) Assumptions & Judgment Calls
- Assumption: enhancing scanner context materially improves generation quality.
- Assumption: additive scanner behavior is safe with existing consumers.
- Judgment call: avoid AST rewrite due scope/risk and no passing baseline in current environment.
- Alternative rejected: broad architecture refactor without executable verification.
