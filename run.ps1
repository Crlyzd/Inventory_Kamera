<#
.SYNOPSIS
    Multi-mode build and run script for Inventory Kamera.

.DESCRIPTION
    Provides automated workflows for running Inventory Kamera:
    - dev / debug : Recompiles incremental Debug build and streams live console logs.
    - release     : Builds optimized Release binaries and packages a portable .zip into dist\.
    - run         : Instantly launches the existing compiled executable without building.

.PARAMETER Mode
    Execution mode: 'dev' (default), 'release', or 'run'.

.PARAMETER NoBuild
    Bypasses compilation and launches the current build directly.

.PARAMETER NoZip
    Skips creating the portable .zip archive in release mode.

.PARAMETER NoLaunch
    Builds (and packages if in release mode) without launching the application GUI.

.PARAMETER Clean
    Performs a clean rebuild of bin/ and obj/ directories.
#>

param (
    [ValidateSet("dev", "debug", "release", "prod", "run")]
    [string]$Mode = "dev",

    [switch]$NoBuild,
    [switch]$NoZip,
    [switch]$NoLaunch,
    [switch]$Clean
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
Set-Location $ScriptDir

# Normalize mode
if ($Mode -in "debug", "dev") {
    $TargetConfig = "Debug"
    $IsDev = $true
} elseif ($Mode -in "release", "prod") {
    $TargetConfig = "Release"
    $IsDev = $false
} else {
    # 'run' mode
    $NoBuild = $true
    $IsDev = $false
    # Prefer Release if present, otherwise Debug
    if (Test-Path "$ScriptDir\InventoryKamera\bin\Release\InventoryKamera.exe") {
        $TargetConfig = "Release"
    } else {
        $TargetConfig = "Debug"
    }
}

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host " Inventory Kamera Runner [$Mode]" -ForegroundColor Cyan
Write-Host " Target Configuration: $TargetConfig" -ForegroundColor DarkGray
Write-Host "============================================================" -ForegroundColor Cyan

function Resolve-MSBuild {
    # 1. Try vswhere.exe
    $vswhereCandidates = @(
        "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe",
        "${env:ProgramFiles}\Microsoft Visual Studio\Installer\vswhere.exe"
    )

    foreach ($vswhere in $vswhereCandidates) {
        if (Test-Path $vswhere) {
            $msbuildPath = & $vswhere -latest -products * -requires Microsoft.Component.MSBuild -find MSBuild\**\Bin\MSBuild.exe | Select-Object -First 1
            if ($msbuildPath -and (Test-Path $msbuildPath)) {
                return $msbuildPath
            }
        }
    }

    # 2. Check PATH
    $cmd = Get-Command msbuild.exe -ErrorAction SilentlyContinue
    if ($cmd) {
        return $cmd.Source
    }

    # 3. Known standard install locations
    $knownPaths = @(
        "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\MSBuild\Current\Bin\MSBuild.exe",
        "C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\MSBuild.exe",
        "C:\Program Files\Microsoft Visual Studio\2022\Professional\MSBuild\Current\Bin\MSBuild.exe",
        "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\MSBuild\Current\Bin\MSBuild.exe"
    )

    foreach ($p in $knownPaths) {
        if (Test-Path $p) {
            return $p
        }
    }

    throw "MSBuild.exe was not found. Please install Visual Studio 2022 or Visual Studio Build Tools with the '.NET desktop development' workload."
}

function Invoke-Build {
    param ([string]$Config)

    $msbuild = Resolve-MSBuild
    Write-Host "[BUILD] Using MSBuild: $msbuild" -ForegroundColor DarkGray

    if ($Clean) {
        Write-Host "[BUILD] Cleaning build artifacts..." -ForegroundColor Yellow
        & $msbuild InventoryKamera.sln -t:Clean -p:Configuration=$Config -p:Platform="Any CPU" -v:m
    }

    Write-Host "[BUILD] Restoring NuGet packages..." -ForegroundColor Cyan
    & $msbuild InventoryKamera.sln -t:restore -p:Configuration=$Config -p:Platform="Any CPU" -v:m
    if ($LASTEXITCODE -ne 0) {
        throw "NuGet package restore failed with exit code $LASTEXITCODE."
    }

    Write-Host "[BUILD] Compiling Inventory Kamera ($Config)..." -ForegroundColor Cyan
    & $msbuild InventoryKamera.sln -p:Configuration=$Config -p:Platform="Any CPU" -v:m
    if ($LASTEXITCODE -ne 0) {
        throw "MSBuild compilation failed with exit code $LASTEXITCODE."
    }

    Write-Host "[BUILD] Build successful!" -ForegroundColor Green
}

function New-ReleasePackage {
    param ([string]$BinDir)

    $distDir = "$ScriptDir\dist"
    if (-not (Test-Path $distDir)) {
        New-Item -ItemType Directory -Path $distDir | Out-Null
    }

    $exePath = "$BinDir\InventoryKamera.exe"
    if (-not (Test-Path $exePath)) {
        throw "Cannot package release: $exePath not found."
    }

    $versionInfo = (Get-Item $exePath).VersionInfo
    $rawVersion = if ($versionInfo.ProductVersion) { $versionInfo.ProductVersion } else { $versionInfo.FileVersion }
    # Clean version string (remove trailing commit hashes if any)
    $version = ($rawVersion -split '\+')[0].Trim()
    if ([string]::IsNullOrWhiteSpace($version)) {
        $version = "portable"
    } else {
        $version = "v$version"
    }

    $zipName = "InventoryKamera-$version-portable.zip"
    $zipPath = Join-Path $distDir $zipName

    Write-Host "[PACKAGE] Packaging portable archive: $zipName..." -ForegroundColor Cyan

    # Stage files in temporary folder to exclude unnecessary items (.pdb, logs, etc.)
    $stagingDir = Join-Path $env:TEMP "IK_Package_$([Guid]::NewGuid().ToString('N'))"
    New-Item -ItemType Directory -Path $stagingDir | Out-Null

    try {
        # Copy binaries, dependencies, configs
        Get-ChildItem -Path $BinDir -Exclude "*.pdb", "*.xml", "logging", "inventorylists" | ForEach-Object {
            Copy-Item -Path $_.FullName -Destination $stagingDir -Recurse -Force
        }

        # Remove existing zip if present
        if (Test-Path $zipPath) {
            Remove-Item $zipPath -Force
        }

        Compress-Archive -Path "$stagingDir\*" -DestinationPath $zipPath -Force

        # Also create/update a generic alias 'InventoryKamera-portable.zip' for convenience
        $aliasZip = Join-Path $distDir "InventoryKamera-portable.zip"
        Copy-Item -Path $zipPath -Destination $aliasZip -Force

        $zipItem = Get-Item $zipPath
        $sizeMB = [math]::Round($zipItem.Length / 1MB, 2)

        Write-Host ""
        Write-Host "============================================================" -ForegroundColor Green
        Write-Host " [PACKAGE SUCCESS] Release portable archive generated!" -ForegroundColor Green
        Write-Host " Archive: $zipPath" -ForegroundColor White
        Write-Host " Size:    $sizeMB MB" -ForegroundColor DarkGray
        Write-Host "============================================================" -ForegroundColor Green
        Write-Host ""
    } finally {
        if (Test-Path $stagingDir) {
            Remove-Item $stagingDir -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}

function Stream-LiveLogs {
    param (
        [System.Diagnostics.Process]$Process,
        [string]$LogFile
    )

    Write-Host "[LOG] Live logger attached. Monitoring: $LogFile" -ForegroundColor Cyan
    Write-Host "[LOG] Press Enter inside Genshin Impact or close the app to stop." -ForegroundColor DarkGray
    Write-Host "------------------------------------------------------------" -ForegroundColor DarkGray

    $isProcessAlive = {
        try {
            if (-not $Process.HasExited) { return $true }
            return $false
        } catch {
            $p = Get-Process -Id $Process.Id -ErrorAction SilentlyContinue
            return ($null -ne $p)
        }
    }

    # Wait up to 5 seconds for log file creation if first run
    $waited = 0
    while (-not (Test-Path $LogFile) -and (& $isProcessAlive) -and $waited -lt 10) {
        Start-Sleep -Milliseconds 500
        $waited++
    }

    if (-not (Test-Path $LogFile)) {
        Write-Host "[LOG] Log file not created yet; waiting for application..." -ForegroundColor DarkGray
        try { $Process.WaitForExit() } catch { Start-Sleep -Seconds 2 }
        return
    }

    # Open log with FileShare.ReadWrite so NLog can write concurrently
    $fileStream = [System.IO.File]::Open($LogFile, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read, [System.IO.FileShare]::ReadWrite)
    $reader = New-Object System.IO.StreamReader($fileStream)
    # Start at end of file for live events
    $fileStream.Seek(0, [System.IO.SeekOrigin]::End) | Out-Null

    try {
        while (& $isProcessAlive) {
            $line = $reader.ReadLine()
            if ($null -ne $line) {
                # Format log levels with colors
                if ($line -match "\|ERROR\|" -or $line -match "\|FATAL\|") {
                    Write-Host $line -ForegroundColor Red
                } elseif ($line -match "\|WARN\|") {
                    Write-Host $line -ForegroundColor Yellow
                } elseif ($line -match "\|INFO\|") {
                    Write-Host $line -ForegroundColor Cyan
                } else {
                    Write-Host $line -ForegroundColor DarkGray
                }
            } else {
                Start-Sleep -Milliseconds 150
            }
        }
    } finally {
        $reader.Close()
        $fileStream.Close()
    }

    Write-Host "------------------------------------------------------------" -ForegroundColor DarkGray
    try {
        Write-Host "[LOG] Application closed (ExitCode: $($Process.ExitCode))." -ForegroundColor Green
    } catch {
        Write-Host "[LOG] Application closed." -ForegroundColor Green
    }
}

# --- MAIN EXECUTION FLOW ---

$BinDir = "$ScriptDir\InventoryKamera\bin\$TargetConfig"
$TargetExe = "$BinDir\InventoryKamera.exe"

# 1. Build step (unless -NoBuild is passed)
if (-not $NoBuild) {
    Invoke-Build -Config $TargetConfig
} else {
    Write-Host "[RUN] Bypassing build. Launching existing binary..." -ForegroundColor Yellow
    if (-not (Test-Path $TargetExe)) {
        Write-Host "[RUN] No existing binary found at $TargetExe. Building first..." -ForegroundColor Yellow
        Invoke-Build -Config $TargetConfig
    }
}

# 2. Release packaging step
if ($TargetConfig -eq "Release" -and -not $NoZip) {
    New-ReleasePackage -BinDir $BinDir
}

# 3. Launch application
if (-not $NoLaunch) {
    if (-not (Test-Path $TargetExe)) {
        throw "Executable not found at $TargetExe."
    }

    Write-Host "[LAUNCH] Starting Inventory Kamera (requesting elevation)..." -ForegroundColor Green
    $startInfo = New-Object System.Diagnostics.ProcessStartInfo
    $startInfo.FileName = $TargetExe
    $startInfo.WorkingDirectory = $BinDir
    $startInfo.UseShellExecute = $true
    $startInfo.Verb = "runas"

    $proc = $null
    try {
        $proc = [System.Diagnostics.Process]::Start($startInfo)
    } catch {
        if ($_.Exception -is [System.ComponentModel.Win32Exception] -and $_.Exception.NativeErrorCode -eq 1223) {
            Write-Host "[LAUNCH] Launch cancelled (UAC elevation prompt declined)." -ForegroundColor Yellow
            return
        }
        # Fallback to non-elevated launch
        $startInfo.Verb = ""
        $proc = [System.Diagnostics.Process]::Start($startInfo)
    }

    if ($null -eq $proc) {
        Write-Host "[LAUNCH] Application launched successfully." -ForegroundColor Green
        return
    }

    if ($IsDev) {
        $logPath = Join-Path $BinDir "logging\InventoryKamera.debug.log"
        Stream-LiveLogs -Process $proc -LogFile $logPath
    } else {
        try {
            Write-Host "[LAUNCH] Process running (PID: $($proc.Id)). Window detached." -ForegroundColor Green
        } catch {
            Write-Host "[LAUNCH] Process running. Window detached." -ForegroundColor Green
        }
    }
} else {
    Write-Host "[DONE] -NoLaunch specified. Workflow completed successfully." -ForegroundColor Green
}
