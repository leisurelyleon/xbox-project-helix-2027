# The "Memory Crisis" Mitigation Strategy

A critical challenge in Project Helix is memory management. Standard PCs have split memory: DDR5 for the CPU, and GDDR6 for the GPU. Xbox uses **Unified Memory Architecture (UMA)**, where 24GB of GDDR7 is shared by everything.

## The Threat
When the `WinHelix Bridge` runs an unmodified PC game, that game will aggressively try to allocate system RAM, entirely unaware that it is eating into the GPU's VRAM. If a PC game consumes all 24GB, the GPU crashes, and the FSR Diamond AI upscaler starves.

## The Resilient Allocator
Our Rust kernel utilizes the `ResilientAllocator.rs` module. It enforces a strict **2GB Critical VRAM Reserve**.

1. **Monitoring:** The kernel intercepts all `VirtualAlloc` calls from the PC game.
2. **Elevation:** If memory usage crosses 75%, the Windows 12 shell is suspended to disk.
3. **Crisis:** If usage hits 90%, the kernel pauses the PC game's threads, executes a physical Translation Lookaside Buffer (TLB) flush, and performs lossless compression on background OS pages.

By spoofing memory limits to the PC game via the `Win32_Shim.c`, the game believes the PC is out of RAM and naturally lowers its own LODs (Level of Detail), protecting the Xbox silicon from crashing.
