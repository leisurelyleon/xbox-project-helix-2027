# ==============================================================================
# PROJECT HELIX: MASTER BUILD ORCHESTRATOR
# Target: Windows 12 / Xbox Native Hybrid
# ==============================================================================

$ErrorActionPreference = "Stop"

Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host "  INITIATING PROJECT HELIX (2027) MASTER BUILD SEQUENCE" -ForegroundColor Cyan
Write-Host "=======================================================" -ForegroundColor Cyan

# 1. Build the Bare-Metal Rust Microvisor
Write-Host "`n[1/4] Compiling Rust Kernel Microvisor (Zen 6 Target)..." -ForegroundColor Yellow
Set-Location -Path "..\kernel_microvisor"
cargo build --release --target x86_64-unknown-none
if ($LASTEXITCODE -ne 0) { throw "Rust compilation failed." }
Write-Host "  -> Kernel successfully compiled." -ForegroundColor Green

# 2. Compile the HLSL Shaders and C++ FSR Diamond Orchestrator
Write-Host "`n[2/4] Compiling GPU Shaders & C++ Rendering Core..." -ForegroundColor Yellow
Set-Location -Path "..\graphics_fsr_diamond"
if (-Not (Test-Path "build")) { New-Item -ItemType Directory -Path "build" | Out-Null }
Set-Location -Path "build"
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . --config Release
if ($LASTEXITCODE -ne 0) { throw "C++/HLSL graphics compilation failed." }
Write-Host "  -> RDNA 5 DXIL Bytecode generated." -ForegroundColor Green

# 3. Compile the C/C++ WinHelix Translation Bridge
Write-Host "`n[3/4] Compiling WinHelix PC Translation Shims..." -ForegroundColor Yellow
Set-Location -Path "..\..\win_helix_bridge"
make all
if ($LASTEXITCODE -ne 0) { throw "WinHelix Bridge compilation failed." }
Write-Host "  -> kernel32.dll and d3d12.dll interceptors ready." -ForegroundColor Green

# 4. Compile the Windows 12 C# / XAML Dashboard (Native AOT)
Write-Host "`n[4/4] Compiling Windows 12 UX Shell (Native AOT)..." -ForegroundColor Yellow
Set-Location -Path "..\helix_dashboard_shell"
dotnet publish -c Release -r win-x64 --self-contained true /p:PublishAot=true
if ($LASTEXITCODE -ne 0) { throw "XAML Shell compilation failed." }
Write-Host "  -> Unified Dashboard compiled to raw machine code." -ForegroundColor Green

Write-Host "`n=======================================================" -ForegroundColor Cyan
Write-Host "  BUILD SUCCESSFUL. READY FOR DEV KIT DEPLOYMENT." -ForegroundColor Cyan
Write-Host "=======================================================" -ForegroundColor Cyan
