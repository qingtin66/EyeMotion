# EyeMotion Unified Build Script
# Usage: .\build_all.ps1

$ErrorActionPreference = "Stop"
$projectRoot = $PSScriptRoot

# 1. Setup Release Directory
$releaseDir = Join-Path $projectRoot "release"
if (Test-Path $releaseDir) {
    Write-Host "🧹 Cleaning up old release artifacts..." -ForegroundColor Yellow
    Remove-Item $releaseDir -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null
Write-Host "📂 Release directory created: $releaseDir" -ForegroundColor Cyan

# Clean previous build artifacts
$oldExe = Join-Path $projectRoot "target\release\eyemotion.exe"
if (Test-Path $oldExe) { Remove-Item $oldExe -Force }

# 2. Build Frontend
Write-Host "🎨 Building Frontend..." -ForegroundColor Yellow
npm run build
if ($LASTEXITCODE -ne 0) { throw "Frontend build failed" }

# 3. Build Windows
Write-Host "🪟 Building Windows..." -ForegroundColor Yellow
npm run tauri build
if ($LASTEXITCODE -ne 0) { throw "Windows build failed" }

# Collect Windows Artifacts
$targetDir = Join-Path $projectRoot "target\release\bundle"
$winSetup = Get-ChildItem "$targetDir\nsis\*.exe" | Select-Object -First 1
$winMsi = Get-ChildItem "$targetDir\msi\*.msi" | Select-Object -First 1
$winExe = Join-Path $projectRoot "target\release\eyemotion.exe"

if ($winSetup) { Copy-Item $winSetup.FullName "$releaseDir\EyeMotion_Windows_Setup.exe" -Force }
if ($winMsi) { Copy-Item $winMsi.FullName "$releaseDir\EyeMotion_Windows_Installer.msi" -Force }

# Create Portable Green Version
if (Test-Path $winExe) {
    Write-Host "📦 Creating Portable Green Version..." -ForegroundColor Yellow
    $portableDir = Join-Path $releaseDir "EyeMotion_Portable_Green"
    New-Item -ItemType Directory -Force -Path $portableDir | Out-Null
    Copy-Item $winExe "$portableDir\EyeMotion.exe" -Force
    $loaderDll = Join-Path $projectRoot "target\release\WebView2Loader.dll"
    if (Test-Path $loaderDll) { Copy-Item $loaderDll "$portableDir\" -Force }
    Compress-Archive -Path "$portableDir\*" -DestinationPath "$releaseDir\EyeMotion_Portable_Green.zip" -Force
    Remove-Item $portableDir -Recurse -Force
}

Write-Host "✅ All builds completed successfully!" -ForegroundColor Green
Write-Host "🚀 Artifacts are available in: $releaseDir" -ForegroundColor Green
Get-ChildItem $releaseDir | Select-Object Name, Length, LastWriteTime | Format-Table -AutoSize
