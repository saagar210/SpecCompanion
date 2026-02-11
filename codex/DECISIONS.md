# Decisions Log

## D-001 — Reduced execution scope due verification blockers
- Date: 2026-02-10T22:52:05+00:00
- Context: Baseline `pnpm build`, `pnpm install`, and `cargo test` all failed due registry/network 403.
- Decision: Proceed with additive, low-risk changes that do not alter DB/API contracts and can be reasoned about statically.
- Alternatives considered:
  - Halt entirely (rejected: user requested end-to-end execution with fallback if blocked).
  - Large refactor (rejected: violates safe/small-change principle without executable baseline).

## D-002 — Improve scanner parity before larger roadmap work
- Date: 2026-02-10T22:52:05+00:00
- Context: Scanner extension list advertises languages without extraction support.
- Decision: Add heuristic extraction for Go/Java/Ruby/C# and tests.
- Rationale: High impact on core generation context, low contract risk.

## D-003 — Keep scanner extraction heuristic and additive
- Date: 2026-02-10T22:52:05+00:00
- Context: Need high-impact improvement with low risk under blocked execution environment.
- Decision: implement lightweight language handlers rather than AST parser rewrite.
- Consequence: immediate parity improvement, while precision limits remain documented as deferred work.
