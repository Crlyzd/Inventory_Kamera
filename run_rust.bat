@echo off
setlocal enabledelayedexpansion
cd /d "%~dp0"

:: ============================================================
:: Inventory Kamera (Rust + egui) Unified Build & Runner
:: Consolidated 1-file runner for Dev, Watch, Fast, Release, Clean
:: ============================================================

:: 1. Verify Rust toolchain
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] 'cargo' was not found in your PATH.
    echo Please install Rust from https://rustup.rs/ before proceeding.
    echo.
    pause
    exit /b 1
)

:: 2. CLI Argument Dispatch
if /i "%1"=="dev" goto :DEV
if /i "%1"=="watch" goto :WATCH
if /i "%1"=="fast" goto :FAST
if /i "%1"=="release" goto :RELEASE
if /i "%1"=="clean" goto :CLEAN
if /i "%1"=="check" goto :CHECK
if not "%1"=="" (
    echo [UNKNOWN OPTION] "%1"
    echo Usage: run_rust.bat [dev ^| watch ^| fast ^| release ^| clean ^| check]
    exit /b 1
)

:: 3. Interactive Menu (when double-clicked or run with no args)
:MENU
cls
echo ============================================================
echo      Inventory Kamera (Rust + egui) Unified Launcher
echo ============================================================
echo.
echo   [1] Live / Dev Mode
echo       - Fast debug compile and run with live terminal output
echo.
echo   [2] Live Watch Mode
echo       - Auto-recompile and run on file save (cargo watch)
echo.
echo   [3] Fast Test Build (Fastest Compile Time)
echo       - Quickest compilation for testing compiled standalone exe
echo       - Skips heavy optimization and LTO (outputs target\debug\*.exe)
echo.
echo   [4] Smallest Portable Release Build
echo       - Full LTO, codegen-units=1, stripped symbols, size-optimized
echo       - Outputs to target\release\inventory_kamera.exe
echo.
echo   [5] Quick Cargo Check
echo       - Fast syntax and type checking without code generation
echo.
echo   [6] Clean Target Directory
echo       - Wipes build artifacts in target\ folder
echo.
echo   [7] Exit
echo.
echo ============================================================
set /p choice="Select an option [1-7]: "

if "%choice%"=="1" goto :DEV
if "%choice%"=="2" goto :WATCH
if "%choice%"=="3" goto :FAST
if "%choice%"=="4" goto :RELEASE
if "%choice%"=="5" goto :CHECK
if "%choice%"=="6" goto :CLEAN
if "%choice%"=="7" exit /b 0

echo Invalid choice, please try again.
pause
goto :MENU

:: ------------------------------------------------------------
:: Targets
:: ------------------------------------------------------------

:DEV
echo.
echo [STARTING] Compiling and running in Development Mode...
cargo run
goto :END

:WATCH
echo.
echo [STARTING] Live Watch Mode...
where cargo-watch >nul 2>&1
if %errorlevel% equ 0 (
    cargo watch -x run
) else (
    echo [INFO] 'cargo-watch' not found. Falling back to standard cargo run.
    echo [TIP] Install cargo-watch anytime via: cargo install cargo-watch
    cargo run
)
goto :END

:FAST
echo.
echo [BUILDING] Compiling Fast Test Standalone Binary (fastest compile time)...
cargo build
if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Fast build failed. Please review compiler errors above.
    goto :END
)

if exist "target\debug\inventory_kamera.exe" (
    echo.
    echo ============================================================
    echo [SUCCESS] Fast test binary ready for testing!
    echo Binary: target\debug\inventory_kamera.exe
    for %%F in ("target\debug\inventory_kamera.exe") do (
        set /a sizeMB=%%~zF/1048576
        echo Size  : ~!sizeMB! MB (%%~zF bytes - unoptimized test build)
    )
    echo Note  : Compiled with parallel codegen for maximum build speed.
    echo ============================================================
) else (
    echo.
    echo [NOTICE] Build finished. Check target\debug for the compiled binary.
)
goto :END

:RELEASE
echo.
echo [BUILDING] Compiling Smallest Portable Release Executable...
echo Options: size-optimized (opt-level="z"), LTO enabled, symbols stripped
cargo build --release
if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Release build failed. Please review compiler errors above.
    goto :END
)

if exist "target\release\inventory_kamera.exe" (
    echo.
    echo ============================================================
    echo [SUCCESS] Smallest portable standalone binary generated!
    echo Binary: target\release\inventory_kamera.exe
    for %%F in ("target\release\inventory_kamera.exe") do (
        set /a sizeMB=%%~zF/1048576
        echo Size  : ~!sizeMB! MB (%%~zF bytes)
    )
    echo ============================================================
) else (
    echo.
    echo [NOTICE] Release build finished. Check target\release for the compiled binary.
)
goto :END

:CHECK
echo.
echo [CHECKING] Running cargo check...
cargo check
goto :END

:CLEAN
echo.
echo [CLEANING] Wiping target\ build directory...
cargo clean
echo [OK] Target directory cleaned.
goto :END

:END
echo.
echo Press any key to exit...
pause >nul
