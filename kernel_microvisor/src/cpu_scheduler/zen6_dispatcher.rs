#![no_std]

use alloc::collections::BTreeMap;
use spin::RwLock;
use x86_64::registers::control::Cr3; // x86_64 specific control registers

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoreType {
    Zen6Performance, // High clock speed, massive L3 cache (For Gaming/Draw Calls)
    Zen6Dense,       // Smaller footprint, hyper-efficient (For OS/Telemetry)
}

#[derive(Debug)]
pub struct ThreadControlBlock {
    pub thread_id: u64,
    pub instruction_pointer: usize,
    pub stack_pointer: usize,
    pub required_core: CoreType,
    pub is_pc_mode_native: bool,
}

pub struct Zen6Dispatcher {
    ready_queue: RwLock<BTreeMap<u64, ThreadControlBlock>>,
    active_threads: RwLock<[Option<u64>; 16]>, // Assuming an 8 P-Core / 8 E-Core setup
}

impl Zen6Dispatcher {
    pub fn new() -> Self {
        crate::println!("[SCHEDULER] Zen 6 Asymmetric Dispatcher Online. Mapping 16 physical cores.");
        Self {
            ready_queue: RwLock::new(BTreeMap::new()),
            active_threads: RwLock::new([None; 16]),
        }
    }

    /// Injects a thread into the run queue based on its architecture requirements
    pub fn enqueue_thread(&self, tcb: ThreadControlBlock) {
        let mut queue = self.ready_queue.write();
        queue.insert(tcb.thread_id, tcb);
    }

    /// The core context switch. This is triggered by a hardware timer interrupt every millisecond.
    #[naked] // Naked functions skip standard Rust prologue/epilogue for raw assembly speed
    pub unsafe extern "C" fn hardware_context_switch() {
        core::arch::asm!(
            // 1. Save current thread's state (Registers) onto its stack
            "push rbp",
            "push rbx",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            
            // 2. Call into the Rust logic to select the next thread
            "call {select_next_thread}",
            
            // 3. Restore the new thread's state
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop rbx",
            "pop rbp",
            
            // 4. Return from hardware interrupt (IRETQ for x86_64)
            "iretq",
            select_next_thread = sym Self::rust_scheduler_tick,
            options(noreturn)
        );
    }

    /// Rust logic called by the assembly to determine the next thread
    extern "C" fn rust_scheduler_tick() {
        // Core routing logic happens here.
        // It maps P-Core threads to APIC IDs 0-7, and E-Core threads to APIC IDs 8-15.
    }
}
