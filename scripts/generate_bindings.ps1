# PowerShell script to generate Kotlin and Swift bindings for Gupt

$ErrorActionPreference = "Stop"
$ProjectRoot = "c:\IMP PROJECTS\Gupt"

Write-Host "Generating UniFFI Bindings..." -ForegroundColor Cyan

# Ensure output directories exist
$AndroidOut = "$ProjectRoot\apps\android\app\src\main\java\com\gupt\ffi"
$IosOut = "$ProjectRoot\apps\ios\GuPT\FFI"

New-Item -ItemType Directory -Force -Path $AndroidOut | Out-Null
New-Item -ItemType Directory -Force -Path $IosOut | Out-Null

# Run the bindgen binary we created in core/ffi
Set-Location $ProjectRoot
cargo run --bin uniffi-bindgen generate core\ffi\src\gupt.udl --language kotlin --out-dir $AndroidOut
cargo run --bin uniffi-bindgen generate core\ffi\src\gupt.udl --language swift --out-dir $IosOut

Write-Host "Bindings successfully generated!" -ForegroundColor Green
Write-Host "Kotlin bindings placed in: $AndroidOut"
Write-Host "Swift bindings placed in: $IosOut"
