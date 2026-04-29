#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod memory;
mod cpu_scheduler;

use core::panic::PanicInfo;
use memory::resilient_allocator::ResilientAllocator;
use cpu_scheduler::zen6_dispatcher::Zen6Dispatcher;
use cpu_scheduler::pc_mode_toggle::{ModeToggle, HelixOperatingMode};

// -------------------------------------------------------------------------
// GLOBAL KERNEL ALLOCATOR
// Maps the full 24GB of GDDR7 Unified Memory, leaving space for the kernel
// -------------------------------------------------------------------------
#[global_allocator]
static ALLOCATOR: ResilientAllocator = ResilientAllocator::new(
    0x2000_0000,                  // Heap Start Address (Physical)
    24 * 1024 * 1024 * 1024       // 24 Gigabytes of GDDR7
);

// -------------------------------------------------------------------------
// BARE-METAL ENTRY POINT
// The bootloader jumps directly to this memory address.
// -------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // In a real bare-metal environment, we would initialize the VGA text buffer
    // or Serial Port here. We assume a 'serial_print!' macro exists for telemetry.

    // 1. SILICON INITIALIZATION
    // ---------------------------------------------------------
    // "serial_printIn!" is our hypothetical bare-metal macro
    // serial_println!("[BOOT] Xbox Project Helix (2027) Microvisor Initializing...");
    // serial_println!("[BOOT] Securing AMD Zen 6 / RDNA 5 Silicon...");

    // 2. MEMORY CRISIS MANAGER
    // ---------------------------------------------------------
    // serial_println!("[BOOT] Calibrating 24GB GDDR7 Unified Memory Pool...");
    ALLOCATOR.evaluate_pressure();

    // 3. DIRECT STORAGE V2 (NVMe -> VRAM)
    // ---------------------------------------------------------
    // serial_println!("[BOOT] Locking PCIe Gen 5 Lanes for NVMe...");
    let mut _direct_storage = memory::direct_storage_v2::DirectStorageV2::new();

    // 4. ZEN 6 ASYMMETRIC SCHEDULER
    // ---------------------------------------------------------
    let _dispatcher = Zen6Dispatcher::new();

    // 5. HYBRID OS TOGGLE (PC / CONSOLE)
    // ---------------------------------------------------------
    let mode_toggle = ModeToggle::new();
    
    // Default boot behavior under Asha Sharma's unified vision: 
    // Boot into the seamless Xbox/Win12 hybrid dashboard.
    if let Err(e) = mode_toggle.initiate_mode_swap(HelixOperatingMode::XboxSandbox) {
        // serial_println!("[BOOT-ERROR] Failed to lock partition: {}", e);
    }

    // serial_println!("[BOOT] Kernel Space Secure. Handing execution to User Space...");
    // serial_println!("[BOOT] Awaiting Dashboard / Windows 12 Shell execution.");

    // 6. HARDWARE HALT LOOP
    // ---------------------------------------------------------
    // The kernel has finished booting. We now put the CPU into a low-power
    // halt state until a hardware interrupt (like a controller input or 
    // a frame render request) wakes it up.
    loop {
        x86_64::instructions::hlt();
    }
}

// -------------------------------------------------------------------------
// HARDWARE EXCEPTION HANDLING
// -------------------------------------------------------------------------

/// This function is called on any unrecoverable Rust panic.
/// On a retail Xbox console, this would trigger the system shutdown/restart loop 
/// and log the telemetry dump to Microsoft servers.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // serial_println!("[FATAL MICROVISOR EXCEPTION]: {}", _info);
    
    // Freeze the CPU instantly to prevent memory corruption
    loop {
        x86_64::instructions::hlt();
    }
}

/// Called if the ResilientAllocator completely runs out of memory,
/// breaching the critical VRAM reserve for FSR Diamond.
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("[MEMORY CRISIS FATAL] Global allocation failed. Requested layout: {:?}", layout)
}  
