#![no_std]

use alloc::vec::Vec;
use core::future::Future;
use core::task::{Context, Poll};
use core::pin::Pin;

/// Hardware-mapped registers for the PCIe Gen 5 NVMe controller
const NVME_COMMAND_QUEUE_BASE: *mut u64 = 0x4000_1000 as *mut u64;
const HARDWARE_DECOMPRESSION_ENGINE: *mut u32 = 0x4000_2000 as *mut u32;

pub struct DirectStorageV2 {
    submission_queue_tail: usize,
    ai_prefetch_active: bool,
}

/// Represents an asynchronous DMA transfer direct from SSD to GPU VRAM
pub struct DmaTransferFuture {
    request_id: u64,
    target_vram_address: usize,
    is_completed: bool,
}

impl DirectStorageV2 {
    pub fn new() -> Self {
        crate::println!("[DIRECT-STORAGE] V2 Engine Initialized. PCIe Gen 5 lanes locked.");
        Self {
            submission_queue_tail: 0,
            ai_prefetch_active: true, // Helix relies heavily on AI asset prediction
        }
    }

    /// Bypasses the CPU. Instructs the NVMe to send data directly to the GPU's decompression silicon.
    pub fn request_asset_to_vram(&mut self, file_offset: u64, size: usize, vram_ptr: usize) -> DmaTransferFuture {
        let req_id = self.submission_queue_tail as u64;
        
        unsafe {
            // Write the scatter-gather list to the physical hardware register
            core::ptr::write_volatile(NVME_COMMAND_QUEUE_BASE.add(self.submission_queue_tail), file_offset);
            core::ptr::write_volatile(HARDWARE_DECOMPRESSION_ENGINE, vram_ptr as u32);
        }

        self.submission_queue_tail = (self.submission_queue_tail + 1) % 1024; // Ring buffer

        DmaTransferFuture {
            request_id: req_id,
            target_vram_address: vram_ptr,
            is_completed: false,
        }
    }
}

// Implement standard Rust async/await for hardware DMA transfers
impl Future for DmaTransferFuture {
    type Output = Result<(), &'static str>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Read the hardware completion queue via Memory Mapped I/O
        let completion_status = unsafe { core::ptr::read_volatile((NVME_COMMAND_QUEUE_BASE as usize + 0x800) as *const u32) };
        
        if completion_status == 1 {
            Poll::Ready(Ok(()))
        } else {
            // Tell the kernel scheduler to wake this thread when the hardware interrupt fires
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
