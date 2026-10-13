# Changelog

All notable changes to Inventory Kamera will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

Important formatting rule: Never use markdown blockquote symbols in this document.

---

## [Unreleased]

### Added
- Project-specific AI agent architecture and workflow directives in `AGENTS.md`.
- Workflow personas in `.agents/workflows/`:
  - `architect.md`: System design, modular decomposition, and atomic planning.
  - `orchestrator.md`: Pipeline coordination and persona delegation.
  - `coding-specialist.md`: Modular Rust + egui coding with single responsibility per file.
  - `code-reviewer.md`: Static review for borderless styling, line ceilings, and DRY/KISS/YAGNI principles.
- Comprehensive technical library and architecture blueprint in `PROJECT_LIBRARY.md`.
- Change history and mistake memory tracker in `history.md`.
- Consolidated 1-click build and live runner script `run_rust.bat` supporting live dev mode, watch mode, fast test compile (fastest turnaround), and smallest size-optimized portable release compilation.

---

## [1.3.16] - Previous Legacy Release

### Fixed
- Addressed font scaling regression during character talent menu OCR.
- Updated database lookup definitions for latest character banner data.
