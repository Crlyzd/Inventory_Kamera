@echo off
setlocal enabledelayedexpansion
cd /d "%~dp0"

:: Check if command line arguments were passed
if not "%~1"=="" (
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0run.ps1" %*
    goto :eof
)

:: Interactive Menu when run without arguments (e.g. double-clicked)
:menu
cls
echo ============================================================
echo               Inventory Kamera Launcher
echo ============================================================
echo.
echo   [1] Live / Dev Mode
echo       - Incremental Debug build
echo       - Streams live OCR and scanner logs to this console
echo.
echo   [2] Release Mode
echo       - Fully optimized Release build
echo       - Automatically packages portable .zip in dist\
echo       - Launches application
echo.
echo   [3] Quick Run (Instant)
echo       - Skips MSBuild entirely
echo       - Launches existing binary immediately (0s)
echo.
echo   [4] Package Release Zip Only
echo       - Builds Release and generates dist\*.zip
echo       - Does NOT open the application window
echo.
echo   [5] Clean Rebuild (Debug)
echo       - Cleans bin/ and obj/ folders before building
echo.
echo   [6] Exit
echo.
echo ============================================================
set /p choice="Select an option [1-6]: "

if "%choice%"=="1" (
    echo.
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0run.ps1" -Mode dev
    goto :end
)
if "%choice%"=="2" (
    echo.
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0run.ps1" -Mode release
    goto :end
)
if "%choice%"=="3" (
    echo.
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0run.ps1" -Mode run
    goto :end
)
if "%choice%"=="4" (
    echo.
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0run.ps1" -Mode release -NoLaunch
    goto :end
)
if "%choice%"=="5" (
    echo.
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0run.ps1" -Mode dev -Clean
    goto :end
)
if "%choice%"=="6" (
    goto :eof
)

echo Invalid choice, please try again.
pause
goto :menu

:end
echo.
echo Press any key to close this window...
pause >nul
