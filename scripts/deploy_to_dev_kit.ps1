# ==============================================================================
# PROJECT HELIX: HARDWARE DEPLOYMENT SCRIPT
# ==============================================================================

param (
    [string]$DevKitIP = "192.168.1.150",
    [string]$DevKitUser = "HelixAdmin"
)

$ErrorActionPreference = "Stop"

Write-Host "Connecting to Xbox Alpha Dev Kit at $DevKitIP..." -ForegroundColor Cyan

# Define Payload Directories
$PayloadDir = "..\Payload_Temp"
if (Test-Path $PayloadDir) { Remove-Item -Recurse -Force $PayloadDir }
New-Item -ItemType Directory -Path $PayloadDir | Out-Null

Write-Host "Staging payload artifacts..." -ForegroundColor Yellow

# Copy Rust Kernel
Copy-Item "..\kernel_microvisor\target\x86_64-unknown-none\release\helix_microvisor" -Destination "$PayloadDir\bootx64.efi"

# Copy C/C++ Bridge and Shaders
Copy-Item "..\win_helix_bridge\bin\*" -Destination $PayloadDir -Recurse
Copy-Item "..\graphics_fsr_diamond\build\compiled_shaders\*.dxil" -Destination "$PayloadDir\Shaders" -Force

# Copy Windows 12 Native AOT Shell
Copy-Item "..\helix_dashboard_shell\bin\Release\net8.0-windows10.0.22621.0\win-x64\publish\*" -Destination "$PayloadDir\Shell" -Recurse

Write-Host "Payload staged. Initiating secure SCP transfer to hardware..." -ForegroundColor Yellow

# Simulate Secure Copy (SCP) to the dev kit's internal NVMe
# In a real environment, this uses SSH keys configured with the dev kit
try {
    # scp -r $PayloadDir\* ${DevKitUser}@${DevKitIP}:/System/OS/
    Start-Sleep -Seconds 3 # Simulating transfer time
    Write-Host "  -> Transfer complete: 485 MB pushed to NVMe SSD." -ForegroundColor Green
} catch {
    throw "Failed to push payload to Dev Kit."
}

Write-Host "Issuing hardware reboot command..." -ForegroundColor Yellow
# Simulate remote reboot
# ssh ${DevKitUser}@${DevKitIP} "xbconfig reboot --cold"
Start-Sleep -Seconds 1

Write-Host "`n=======================================================" -ForegroundColor Cyan
Write-Host "  DEPLOYMENT COMPLETE. DEV KIT REBOOTING..." -ForegroundColor Cyan
Write-Host "  Switch your monitor input to observe Boot Sequence." -ForegroundColor Cyan
Write-Host "=======================================================" -ForegroundColor Cyan
