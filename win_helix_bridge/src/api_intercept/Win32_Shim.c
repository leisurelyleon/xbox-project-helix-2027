#include <windows.h>
#include <stdio.h>
#include <stdint.h>

// External hooks into our Rust Microvisor
extern void* microvisor_allocate_memory(size_t size, uint32_t protection_flags);
extern uint64_t microvisor_create_thread(void* start_address, void* parameter);

// -------------------------------------------------------------------------
// MEMORY MANAGEMENT SHIMS
// -------------------------------------------------------------------------

/**
 * Intercepts VirtualAlloc. PC games use this to grab RAM.
 * We reroute this to our ResilientAllocator to prevent the "Memory Crisis".
 */
LPVOID WINAPI Shim_VirtualAlloc(LPVOID lpAddress, SIZE_T dwSize, DWORD flAllocationType, DWORD flProtect) {
    printf("[WIN32-SHIM] Intercepted VirtualAlloc request for %zu bytes.\n", dwSize);
    
    // Pass the request down to the bare-metal Rust allocator
    void* allocated_memory = microvisor_allocate_memory(dwSize, flProtect);
    
    if (!allocated_memory) {
        printf("[WIN32-SHIM] CRITICAL: Microvisor denied memory allocation. Triggering OOM spoof.\n");
        SetLastError(ERROR_OUTOFMEMORY);
        return NULL;
    }
    
    return allocated_memory;
}

// -------------------------------------------------------------------------
// THREADING SHIMS
// -------------------------------------------------------------------------

/**
 * Intercepts CreateThread. PC games use this to spawn worker threads.
 * We reroute this to our Zen6Dispatcher to force the thread onto the correct P-Core or E-Core.
 */
HANDLE WINAPI Shim_CreateThread(
    LPSECURITY_ATTRIBUTES lpThreadAttributes,
    SIZE_T dwStackSize,
    LPTHREAD_START_ROUTINE lpStartAddress,
    LPVOID lpParameter,
    DWORD dwCreationFlags,
    LPDWORD lpThreadId
) {
    printf("[WIN32-SHIM] Intercepted CreateThread. Routing to Zen 6 Asymmetric Scheduler...\n");
    
    // The Microvisor returns a bare-metal hardware thread ID
    uint64_t hardware_tid = microvisor_create_thread((void*)lpStartAddress, lpParameter);
    
    if (lpThreadId) {
        *lpThreadId = (DWORD)hardware_tid;
    }
    
    // Return a fake Windows HANDLE so the PC game doesn't crash
    return (HANDLE)hardware_tid; 
}
