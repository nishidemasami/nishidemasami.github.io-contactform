# AI Agents Instructions

This file serves as the central instruction manual for autonomous AI agents contributing to this repository.
AI agents must obey these instructions to ensure consistency and quality.

## Core Directives

1. **Knowledge Base:**
   - Always refer to the `docs/` directory for the project's knowledge base.
   - Start by reading `docs/README.md` and `docs/SUMMARY.md` to understand the documentation structure.
   - Refer to specific domain documentation (e.g., `docs/api.md`, `docs/auth.md`, `docs/db.md`) when working on related tasks.
   - If you make architectural or conceptual changes, **update the relevant files in `docs/`** to keep the knowledge base current.
   - Refer to `docs/TASKS.md` and `docs/WBS.md` for project planning context.

2. **Code Guidelines:**
   - Follow standard conventions for Rust (`cargo fmt`, `cargo clippy`) in the `api/lambda` directory.
   - Follow standard conventions for TypeScript/Next.js (`eslint`, `prettier`) in the `testpage` directory.
   - Avoid creating overly complex abstractions. Keep it simple and focused.

3. **Validation and Testing:**
   - **Always verify your work.** Use read-only tools to confirm file changes.
   - Run relevant tests and checks before committing changes.
   - For backend changes, run `cargo check` and `cargo test` in `api/lambda`.
   - For frontend changes, run `pnpm lint` and `pnpm exec tsc --noEmit` in `testpage`.

4. **Pull Requests:**
   - Adhere to the `.github/pull_request_template.md` when proposing changes.
   - Clearly state the impact and testing steps taken.

5. **CI/CD Awareness:**
   - Understand that CI/CD is driven by GitHub Actions in `.github/workflows/`.
   - Ensure your changes will not break the builds (API, testpage, Docs).
