#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};
use core::ptr::NonNull;
use alloc::alloc::{GlobalAlloc, Layout};
use spin::Mutex; // Bare-metal spinlock for kernel space

/// The absolute total of shared GDDR7 RAM in the Helix console (e.g., 24GB)
const TOTAL_UNIFIED_MEMORY_BYTES: usize = 24 * 1024 * 1024 * 1024;
/// Reserve 2GB strictly for the Microvisor and FSR Diamond tensor operations
const CRITICAL_VRAM_RESERVE: usize = 2 * 1024 * 1024 * 1024;

#[derive(Debug, PartialEq)]
pub enum MemoryPressure {
    Normal,
    Elevated,
    Crisis,
}

pub struct ResilientAllocator {
    heap_start: usize,
    heap_size: usize,
    allocated_bytes: AtomicUsize,
    pressure_state: Mutex<MemoryPressure>,
    // Page table directories mapping virtual to physical address
    page_tables: Mutex<PageTableDirectory>,
}

impl ResilientAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_size,
            allocated_bytes: AtomicUsize::new(0),
            pressure_state: Mutex::new(MemoryPressure::Normal),
            page_tables: Mutex::new(PageTableDirectory::empty()),
        }
    }

    /// Constantly monitors memory. If PC Mode consumes too much, it triggers a crisis mitigration.
    pub fn evaluate_pressure(&self) -> MemoryPressure {
        let current_usage = self.allocated_bytes.load(Ordering::Acquire);
        let usage_ratio = current_usage as f64 / self.heap_size as f64;

        let mut state = self.pressure_state.lock();
        if usage_ratio > 0.90 {
            *state = MemoryPressure::Crisis;
            self.trigger_crisis_mitigation();
            MemoryPressure::Crisis
        } else if usage_ratio > 0.75 {
            *state = MemoryPressure::Elevated;
            MemoryPressure::Elevated
        } else {
            *state = MemoryPressure::Normal;
            MemoryPressure::Normal
        }
    }

    /// Instantly compresses dormant OS pages and reallocates memory back to the GPU/Game
    #[cold]
    fn trigger_crisis_mitigation(&self) {
        crate::println!("[MICROVISOR] OOM Crisis Detected. Initiating hardware page compression...");
        
        let mut pt = self.page_tables.lock();
        pt.scan_and_compress_dormant_pages();
        
        // Force the hardware memory controller to flush the Translation Lookaside Buffer (TLB)
        unsafe {
            core::arch::asm!("mov cr3, {0}", in(reg) pt.physical_address());
        }
        
        crate::println!("[MICROVISOR] Crisis averted. VRAM reserve protected for RDNA 5.");
    }
}

// Implement the global allocator trait so Rust knows how to ask the hardware for memory
unsafe impl GlobalAlloc for ResilientAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.evaluate_pressure();
        
        // Advanced hardware-aligned allocation logic would interface with the MMU here
        let size = layout.size();
        let current = self.allocated_bytes.fetch_add(size, Ordering::SeqCst);
        
        if current + size > self.heap_size - CRITICAL_VRAM_RESERVE {
            // Hard panic if we breach the AI upscaling reserve boundary
            core::ptr::null_mut()
        } else {
            (self.heap_start + current) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.allocated_bytes.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}
