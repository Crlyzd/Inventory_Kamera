# Project History & Memory Log

This document tracks all meaningful code changes, architectural decisions, and registered mistakes. **Every agent persona MUST inspect this file before proposing or executing a plan.**

---

## Mistake Prevention Protocol
Whenever the user states something resembling *"you made a mistake"*, *"that is wrong"*, *"don't do that"*, or points out an error:
1. Immediately log an entry below tagged **`[MISTAKE]`**.
2. Document:
   - **Date & Context**: What task was being performed.
   - **Error Description**: What went wrong.
   - **Root Cause**: Why it happened.
   - **Prevention Rule**: Strict rule to prevent this from ever happening again.
3. Every agent reading this log MUST enforce all active prevention rules in future steps.

---

## Mistake Memory Log

*(No mistakes recorded yet. Keep this section updated whenever a mistake is flagged by the user.)*

---

## Change History Log

### [2026-10-13] — Agent System & Architecture Framework Initialization
- **Action**: Initialized the project-specific agent framework for developing and maintaining Inventory Kamera in Rust + egui.
- **Created Files**:
  - `AGENTS.md`: Master operating principles, UI standards, line ceilings, git & release protocols.
  - `.agents/workflows/architect.md`: Architect planning persona.
  - `.agents/workflows/orchestrator.md`: Task coordination and handoff persona.
  - `.agents/workflows/coding-specialist.md`: Single-responsibility Rust implementation persona.
  - `.agents/workflows/code-reviewer.md`: Borderless UI, line limit, and modularity auditing persona.
  - `PROJECT_LIBRARY.md`: Comprehensive technical manual, architecture diagram, and directory tree.
  - `history.md`: Change history and error memory log.
  - `CHANGELOG.md`: Clean changelog format without blockquotes (`>`).
  - `run_rust.bat`: Consolidated 1-click dev, watch, check, clean, and release compiler.
- **Consolidation & Build Modes**: Removed `run_rust.ps1` and consolidated all tasks into `run_rust.bat`, adding a dedicated Fast Test Build (`run_rust.bat fast` for quickest compile time to test standalone `.exe` behavior) and Smallest Portable Release Build (`run_rust.bat release` with `opt-level="z"`, LTO, codegen-units=1, and symbol stripping).
- **Git Commit Protocol Enhanced**: Mandated dual rationales (per-file reasoning + batch-level justification for committing that specific group of files at that instance), proactive direct commit execution by the agent, and mandatory generation of rich Markdown commit documentation in Artifacts (leaving chat uncluttered).
- **Release Notes Relocated to Artifacts**: Removed `RELEASE_NOTES.md` from the project repository to prevent cluttering the project folder; all standalone GitHub release note documents are now generated as Artifacts.
- **Note**: No application code modified yet, per user instruction.
