#![no_std]

use spin::Mutex;
use x86_64::instructions::interrupts;

#[derive(Debug, PartialEq)]
pub enum HelixOperatingMode {
    XboxSandbox,       // Locked down, highly optimized API footprint
    NativeWindows12,   // Full Win32 API access, unrestricted PC mode
    Transitioning,
}

pub struct ModeToggle {
    current_mode: Mutex<HelixOperatingMode>,
}

impl ModeToggle {
    pub const fn new() -> Self {
        Self {
            current_mode: Mutex::new(HelixOperatingMode::XboxSandbox),
        }
    }

    /// Executes the bare-metal transition between Console and PC paradigms
    pub fn initiate_mode_swap(&self, target_mode: HelixOperatingMode) -> Result<(), &'static str> {
        let mut mode = self.current_mode.lock();
        
        if *mode == target_mode {
            return Err("Target mode is already active.");
        }

        *mode = HelixOperatingMode::Transitioning;
        crate::println!("[MICROVISOR] Initiating Hyper-V Partition Swap. Unmasking hardware...");

        // Disable CPU interrupts to prevent crashing while we swap out the OS underneath the user
        interrupts::without_interrupts(|| {
            unsafe {
                if target_mode == HelixOperatingMode::NativeWindows12 {
                    self.unlock_win32_silicon();
                } else {
                    self.lock_to_xbox_sandbox();
                }
            }
        });

        *mode = target_mode;
        crate::println!("[MICROVISOR] Transition Complete. Helix Architecture Realigned.");
        Ok(())
    }

    /// PC Mode requires full access to the file system and unrestricted CPU ring privileges
    unsafe fn unlock_win32_silicon(&self) {
        // Adjust AMD SEV-SNP (Secure Encrypted Virtualization) VMPL privileges
        // to allow the Windows partition to talk directly to the hardware.
        let vmpl_control_register: *mut u64 = 0xC001_0114 as *mut u64;
        core::ptr::write_volatile(vmpl_control_register, 0x1); // Set to highest privilege
    }

    /// Xbox Mode locks the hardware down for security and raw, unadulterated gaming performance
    unsafe fn lock_to_xbox_sandbox(&self) {
        let vmpl_control_register: *mut u64 = 0xC001_0114 as *mut u64;
        core::ptr::write_volatile(vmpl_control_register, 0x3); // Set to restrictive guest privilege
    }
}
