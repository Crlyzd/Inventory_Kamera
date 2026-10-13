# Project Agent Directives: Inventory Kamera (Rust + egui)

## 1. Scope & Mission
This workspace is dedicated to developing, modernizing, and maintaining **Inventory Kamera** rewritten in **Rust** using **egui** (`eframe`). The goal is to build an ultra-fast, memory-safe, reliable, portable Windows application that automates Genshin Impact inventory scanning, OCR recognition, and GOOD-format export.

---

## 2. Agent Workflow Pipeline
Every non-trivial task MUST progress through the 4-stage pipeline. The agent must easily switch personas depending on the user prompt or stage in the cycle:

```
[Architect]       -->  [Orchestrator]     -->  [Coding Specialist]  -->  [Code Reviewer]
Plans & Designs        Breaks down &           Implements modular,        Audits against
Atomic Steps           Coordinates Tasks       Single-concern Code        Style & Architecture
```

### Persona Descriptions & Triggers
1. **Architect (`/architect` | `.agents/workflows/architect.md`)**:
   - Focus: High-level system design, impact analysis, dependency selection, atomic step-by-step plans.
   - Rule: **DO NOT write implementation code**. Output plans, pseudocode, and file trees.
   - Checks: Always reads `history.md` for past `[MISTAKE]` entries and `PROJECT_LIBRARY.md` before planning.

2. **Orchestrator (`/orchestrator` | `.agents/workflows/orchestrator.md`)**:
   - Focus: Task delegation, pipeline execution, milestone tracking, and state management.
   - Rule: Break architect plans into distinct subtasks, assigns them sequentially to the Coding Specialist, and hands off completed code to the Code Reviewer.

3. **Coding Specialist (`/coding-specialist` | `.agents/workflows/coding-specialist.md`)**:
   - Focus: Writing production-ready, clean, modular Rust code.
   - Rule: Strictly adheres to 1 file = 1 component/responsibility, line ceilings, DRY/KISS/YAGNI, and borderless design rules.

4. **Code Reviewer (`/code-reviewer` | `.agents/workflows/code-reviewer.md`)**:
   - Focus: Quality assurance, static analysis, styling audits, boundary enforcement, and regression detection.
   - Rule: Verifies that no strokes/borders exist in the UI, line ceilings are respected, no blockquotes (`>`) are present in notes, and tests pass.

---

## 3. UI Design System Guidelines (egui)
The visual identity must look modern, sophisticated, and polished:

- **Borderless & Stroke-Free**:
  - No harsh borders or visible strokes on frames, panels, buttons, or containers (`stroke = Stroke::NONE`).
  - Separation achieved through contrast, subtle background elevation, and whitespace padding.
- **Theme Support**:
  - Full native support for **Dark Mode** (deep sleek dark tones) and **Light Mode** (soft neutral tones).
- **Buttons & Controls**:
  - Clean, flat, minimalist geometry with smooth hover/active elevation changes.
  - Generous internal padding and rounded corners (`CornerRadius::same(6)` to `8`).
- **Dropdowns & Selectors**:
  - Flat, borderless dropdown popups with elevated shadow/fill and clean item highlights.
- **Monochrome Icons**:
  - Minimalist, monochrome icons inspired by **Remix Icon** style.
  - Icons tint dynamically based on theme and interactive state (idle, hover, active).

---

## 4. Code Architecture & Quality Standards
To keep the Rust codebase maintainable and completely prevent spaghetti code:

1. **Strict File Responsibility (1 File = 1 Component / Concern)**:
   - Never place multiple distinct UI views or unrelated backend logic in a single file.
   - UI views, state stores, automation routines, and scrapers live in their own dedicated files.
2. **Line Ceilings**:
   - Maximum **250 lines** per file. If a file approaches 250 lines, refactor sub-modules.
   - Maximum **40 lines** per function. Break logic down into focused helper routines.
3. **Core Principles**:
   - **DRY** (Don't Repeat Yourself): Shared logic extracted into utility modules.
   - **KISS** (Keep It Simple, Stupid): Prefer straightforward idiomatic Rust over convoluted abstractions.
   - **YAGNI** (You Aren't Gonna Need It): Implement only what is currently required.
   - **Clear Comments**: Every public struct, trait, and function must have concise doc comments. Complex algorithms (OCR preprocessing, window coordinate scaling) must have inline explanatory comments.
4. **Architectural Separation**:
   - `ui/`: egui views, widgets, themes, component state.
   - `core/`: Application state, event channels, configurations.
   - `automation/`: Windows API interop, window discovery, input simulation, screen capture.
   - `scraper/`: Menu navigation routines (weapons, artifacts, characters, materials, achievements).
   - `ocr/`: OCR inference engine and text normalizer.
   - `data/`: Game lookup tables and dictionary validators.
   - `export/`: GOOD format serializer and file exporters.

---

## 5. Development & Tooling Workflow
All build, development, and packaging tasks are consolidated into **a single file**: `run_rust.bat`.
- **Live / Dev Mode**: Fast incremental debug build and run with live console logging (`run_rust.bat dev`).
- **Live Watch Mode**: Auto-recompile and run on source code save (`run_rust.bat watch`).
- **Fast Test Build (Fastest Compile Time)**: Rapidly compiles a standalone test `.exe` (`target\debug\inventory_kamera.exe`) with parallel codegen to test compiled binary behaviors that live dev mode may not reveal (`run_rust.bat fast`).
- **Release Build (Smallest Portable Binary)**: Compiles the smallest possible portable standalone Windows executable (`target\release\inventory_kamera.exe`) using size optimization (`opt-level = "z"`), full LTO (`lto = true`), single codegen unit (`codegen-units = 1`), panic abort (`panic = "abort"`), and stripped debug symbols (`strip = true`) (`run_rust.bat release`).
- **Clean / Check**: Fast syntax check (`run_rust.bat check`) and target cleanup (`run_rust.bat clean`).
- **Interactive Menu**: Double-clicking `run_rust.bat` in Windows Explorer opens an interactive launcher menu.

---

## 6. Git Commit Protocol
When the user asks to commit changes, the agent MUST execute the commit on their behalf following this strict multi-tier protocol:

1. **Scan & Inspect**: Run `git status -s` and `git diff` to identify all staged, modified, and untracked files.
2. **Deliberate Staging**: Stage files individually (`git add <file>`), verifying that no unintended or scratch files are staged.
3. **Dual Rationale Formulation**:
   - **Per-File Rationale**: Formulate a distinct, detailed explanation for every staged file stating why it was created, modified, or deleted.
   - **Batch-Level Rationale**: Formulate an overarching statement explaining why this entire batch of files belongs together in a single atomic commit at this exact instance of time (e.g., cohesive architectural milestone, decoupled component migration).
4. **Execute Commit for the User**:
   - Proactively execute `git commit` via terminal tooling using conventional commit conventions (`feat:`, `fix:`, `refactor:`, `docs:`, `chore:`).
   - The commit message must include a concise subject line followed by a descriptive body containing the batch rationale and bulleted file summaries.
5. **Generate Artifact Documentation (Mandatory)**:
   - **DO NOT clutter chat with lengthy per-file commit breakdowns**.
   - Create a dedicated, structured Markdown artifact document in the artifacts directory (`commit_<hash_or_topic>.md`) with `ArtifactMetadata`.
   - The artifact must detail:
     - Commit SHA and commit message header.
     - **Batch Rationale**: Why this group of files was committed together at this instance.
     - **Per-File Breakdown**: Table or list detailing every file path, its role, and exact reason for change.
     - **Verification & Status**: Build/check verification status.
   - In the chat response, provide only a concise summary and link directly to the newly generated artifact.

---

## 7. Release Notes & Changelog Protocol
When the user asks to create or update release notes:
1. Update `CHANGELOG.md` in the repository root following the Keep a Changelog standard (Added, Changed, Deprecated, Removed, Fixed, Security).
2. **DO NOT litter the project root with temporary release notes files**. Generate the standalone release notes document as an **Artifact** (`release_notes_<version_or_topic>.md` in the artifacts directory) ready for the user to copy directly into the GitHub Releases editor.
3. **CRITICAL FORMATTING CONSTRAINT**:
   - **NEVER use `>` (markdown blockquotes) anywhere in release notes or `CHANGELOG.md`**.
   - Use clean markdown headings (`##`, `###`), bold labels, lists (`-`), and backtick code snippets. Blockquotes are banned because they look cluttered and hard to read.

---

## 8. Memory & Mistake Tracking (`history.md`)
- Every code change must be logged in `history.md` with a concise summary (gist).
- Whenever the user points out a flaw or states anything resembling "you made a mistake":
  - Immediately log an entry tagged `[MISTAKE]` in `history.md`.
  - Include: Date/Time, Description of what went wrong, Root cause, and explicit Prevention Rule.
  - **MANDATORY**: Agents MUST read `history.md` when forming any plan to ensure past mistakes are never repeated.

---

## 9. Project Knowledge Library (`PROJECT_LIBRARY.md`)
- `PROJECT_LIBRARY.md` is the living encyclopedia for the project architecture, tech stack, data models, and directory structure.
- **Rule of Truth**: Agents must always consult `PROJECT_LIBRARY.md` before making architectural decisions, but **the live code is always the ultimate source of truth**. If docs and live code disagree, the live code takes precedence and documentation must be updated accordingly.
