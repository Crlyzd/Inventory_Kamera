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

### [MISTAKE] [2026-10-14] — Batch Commit Instead of 1 File Per Commit
- **Context**: User asked to create a new branch and commit the Rust project changes.
- **Description**: All 47 files were staged and committed in a single batch commit (`453129b`) with a separate markdown artifact document, instead of making atomic commits 1 file at a time with the per-file rationale written directly in each individual commit message.
- **Root Cause**: Misinterpreted the Git commit protocol's batch documentation artifact as the primary deliverable rather than creating granular 1-file-per-commit entries with descriptive rationales embedded in each commit message.
- **Prevention Rule**: Always commit 1 file at a time, writing the exact per-file role, purpose, and rationale directly into each commit message subject and body. Never combine multiple new/modified files into a single batch commit unless explicitly requested by the user.

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

### [2026-10-14] — Phase 1: Rust + egui Foundation & Frameless 800x600 Shell
- **Action**: Scaffolded the Rust 2021 codebase with `eframe` 0.31, borderless design tokens, and a strictly non-resizable 800x600 frameless window shell.
- **Created Files**:
  - `Cargo.toml`: Package setup, eframe 0.31 glow/accesskit backend, and release optimization profile.
  - `src/main.rs`: Entrypoint with non-resizable 800x600 frameless window and self-update `.old` cleanup.
  - `src/app.rs`: Root `eframe::App` controller managing layout panels.
  - `src/core/mod.rs`, `src/core/state.rs`, `src/core/config.rs`: Reactive state models and persistent settings.
  - `src/ui/mod.rs`, `src/ui/theme.rs`: Borderless theme engine with `Stroke::NONE` and Dark/Light palettes.
  - `src/ui/components/titlebar_button.rs`: Frameless minimize and close window controls (no maximize).
  - `src/ui/components/chip.rs`: Interactive category toggle pills (`[ ⚔ Weapons ]`, etc.).
  - `src/ui/components/button.rs`: Flat borderless action buttons.
  - `src/ui/views/header_bar.rs`: Custom draggable header bar with game status, utility buttons, and window controls.
  - `src/ui/views/chip_bar.rs`: Category toggle chip bar with quick rarity/level filters.
  - `src/ui/views/ocr_stage.rs`: Full-bleed central viewport with idle ready screen and proportional scan card.
  - `src/ui/views/action_dock.rs`: Floating bottom action dock with Start/Stop and Exports buttons.
- **Verification**: `cargo check` and `run_rust.bat check` pass cleanly with 0 errors and 0 warnings. All files strictly adhere to the ≤ 250 lines ceiling and ≤ 40 lines per function rule.

### [2026-10-14] — Phase 2: Dedicated Settings Window & About Tab
- **Action**: Implemented the floating Settings modal dialog with General, Capture, and About tabs.
- **Created / Modified Files**:
  - `src/core/state.rs`: Added `SettingsTab` enum and `CaptureSettings` model (WGC/BitBlt, delays, OCR threads).
  - `src/ui/views/settings/mod.rs`: Re-exports for settings components.
  - `src/ui/views/settings/window.rs`: Floating modal shell with backdrop dimming, tab pill switcher, and close button.
  - `src/ui/views/settings/tab_general.rs`: Export path browser, auto-open exports, and notification toggles.
  - `src/ui/views/settings/tab_capture.rs`: WGC vs BitBlt radio buttons, traversal/scroll delay sliders, OCR worker threads, and confidence sliders.
  - `src/ui/views/settings/tab_about.rs`: Author tribute to Kaleksanan Bagus (@Crlyzd), upstream fork to Rogerio Hirooka (@rogerioth), and original author Andrewthe13th with clickable links, update check, and GitHub Issues form opener.
  - `src/ui/views/action_dock.rs` & `src/ui/views/header_bar.rs`: Implemented defensive directory creation before calling `open::that("./exports")`.
  - `src/app.rs`: Integrated `render_settings_window` into main app event loop.
- **Verification**: `cargo check` and `run_rust.bat check` pass with 0 errors and 0 warnings. All files strictly ≤ 138 lines (well below the 250-line limit); all functions ≤ 40 lines.

### [2026-10-14] — Phase 3: Reactive State, Channels & Standalone Logs Window
- **Action**: Implemented thread-safe diagnostic logging (`LogStore`), cross-thread event bus (`EventBus`), and the standalone diagnostics log modal window (`render_log_window`).
- **Created / Modified Files**:
  - `src/core/logger.rs`: Structured in-memory ring buffer (up to 1,000 entries) with severity levels (`Info`, `Warning`, `Error`, `Scan`) and formatted timestamps.
  - `src/core/events.rs`: Bi-directional cross-thread messaging channels (`ScanCommand` and `ScanEvent`) using `crossbeam_channel`.
  - `src/ui/views/log_window.rs`: Standalone diagnostics window with severity pills, copy-to-clipboard, auto-scroll, and log clearance.
  - `src/ui/views/mod.rs`: Re-exported `render_log_window`.
  - `src/app.rs`: Bound `LogStore` and `EventBus` to the root application state and hooked initial startup logs.
- **Verification**: `cargo check` passes cleanly with 0 errors and 0 warnings. All files strictly under 100 lines (well below the 250-line limit); all functions under 40 lines.

### [2026-10-14] — Phase 4: Multi-Arch Self-Updater Engine & Windows Rename Swap
- **Action**: Implemented the standalone self-updater engine with multi-architecture asset recognition and atomic in-place Windows executable replacement.
- **Created / Modified Files**:
  - `Cargo.toml`: Added `reqwest` with blocking, json, and rustls-tls features.
  - `src/core/self_updater.rs`: Multi-architecture asset matcher (`x86_64`, `aarch64`, `x86`), dynamic rename-proof executable location (`std::env::current_exe()`), download streamer, atomic `.old` rename swap, auto-relaunch with arguments, and clean exit.
  - `src/core/mod.rs`: Re-exported `check_for_updates`, `apply_update`, and `UpdateStatus`.
- **Verification**: `cargo check` passes with 0 errors and 0 warnings. All functions ≤ 30 lines (well below 40-line ceiling); file is 143 lines (well below 250-line ceiling).

### [2026-10-14] — Phase 5: Win32 Window Discovery & Screen Automation
- **Action**: Implemented native Windows API window detection (`FindWindowW`), DPI client bounds extraction, aspect ratio validation, and DirectInput virtual keyboard simulation (`SendInput`).
- **Created / Modified Files**:
  - `Cargo.toml`: Added Windows API crate features (`Win32_Foundation`, `Win32_UI_WindowsAndMessaging`, `Win32_UI_Input_KeyboardAndMouse`, `Win32_Graphics_Gdi`).
  - `src/automation/window_finder.rs`: Discovers active Genshin Impact window handle and extracts client rect/position via `ClientToScreen` and `GetClientRect`.
  - `src/automation/window_scaler.rs`: Aspect ratio validation (16:9 and 16:10) and normalized crop coordinate calculator (`GenshinCrops`).
  - `src/automation/input_simulator.rs`: DirectInput scan-code keyboard driver sending hardware keys (`KEY_B`, `KEY_C`, `ESCAPE`, arrows) with configurable hold durations.
  - `src/automation/mod.rs`: Re-exports automation routines.
  - `src/app.rs`: Connected live window discovery to reactive `state.game_window` with diagnostic logging on detection.
- **Verification**: `cargo check` and `run_rust.bat check` pass with 0 errors and 0 warnings. All files strictly ≤ 78 lines (well below the 250-line limit); all functions ≤ 40 lines.

### [2026-10-14] — Phase 6: OCR Preprocessor & Fuzzy Dictionary Matching
- **Action**: Implemented game data entities, defensive dictionary repository loading, fast Jaro-Winkler string similarity matching (`strsim`), and OCR text sanitization.
- **Created / Modified Files**:
  - `Cargo.toml`: Added `strsim` dependency for fuzzy string matching.
  - `src/data/models.rs`: Definitions for `GameWeapon`, `GameArtifact`, `GameCharacter`, and `StatKey` with standard GOOD keys.
  - `src/data/matcher.rs`: Jaro-Winkler similarity matcher with confidence thresholding for correcting OCR recognition misreads.
  - `src/data/repository.rs`: Defensive lookup repository loading embedded defaults with graceful handling of missing disk lists.
  - `src/data/mod.rs`: Re-exports for data models and matching utilities.
  - `src/ocr/normalizer.rs`: Text sanitization, stat value extraction, and label canonicalization.
  - `src/ocr/mod.rs`: Re-exports for OCR normalizer.
  - `src/main.rs`: Registered `data` and `ocr` submodules.
- **Verification**: `cargo check` passes cleanly with 0 errors and 0 warnings. All files strictly under 75 lines (well below the 250-line limit); all functions under 40 lines.

### [2026-10-14] — Phase 7 & 8: Menu Scrapers & GOOD v2 JSON Export
- **Action**: Implemented menu traversal state machines (`WeaponWalker`, `ArtifactWalker`) and the standardized GOOD v2 JSON database serializer.
- **Created / Modified Files**:
  - `src/scraper/trait_scraper.rs`: Core `Scraper` trait and `ScraperStepResult` enum.
  - `src/scraper/weapon_walker.rs`: Weapon grid traversal with DirectInput arrow navigation.
  - `src/scraper/artifact_walker.rs`: Artifact grid traversal with rarity filtering.
  - `src/scraper/mod.rs`: Re-exports for scrapers.
  - `src/export/good_format.rs`: Standard GOOD v2 database schema (`GoodDatabase`, `GoodWeapon`, `GoodArtifact`, `GoodSubstat`).
  - `src/export/json_serializer.rs`: Pretty JSON serializer saving to `exports/genshin_good_export_{timestamp}.json` with automatic directory creation.
  - `src/export/mod.rs`: Re-exports for export serializer.
  - `src/main.rs`: Registered `scraper` and `export` modules.
- **Verification**: `cargo check` and `run_rust.bat check` pass with 0 errors and 0 warnings. All files across the entire codebase strictly adhere to ≤ 143 lines (far below 250 limit) and ≤ 35 lines per function (below 40 limit).

### [2026-10-14] — Tooling Refinement: Unified Runner CLI Ergonomics
- **Action**: Optimized `run_rust.bat` to detect interactive vs CLI execution.
- **Created / Modified Files**:
  - `run_rust.bat`: Added `INTERACTIVE` flag. Script returns immediately to prompt on CLI dispatch (`check`, `fast`, `dev`, `release`, `clean`) while preserving `pause >nul` when launched via interactive double-click menu.
- **Verification**: `cmd /c run_rust.bat check` verified live; exits immediately with code 0 in 0.57s without hanging on keypress.







