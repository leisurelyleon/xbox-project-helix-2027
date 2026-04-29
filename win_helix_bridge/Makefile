# WinHelix Bridge - Cross-Compilation Makefule
# Targets the custom Project Helix Architecture (x86_64 / Zen 6)

CXX = clang++
CC = clang

# Aggressive optimizations and strictly no standard libraries where possible
CXXFLAGS = -std=c++20 -O3 -Wall -Wextra -fno-exceptions -fno-rtti -masm=intel
CFLAGS = -std=c11 -O3 -Wall -Wextra

# Include directories
INCLUDES = -I./src -I../kernel_microvisor/include

# Output binaries
OUT_DIR = bin
BRIDGE_EXE = $(OUT_DIR)/win_helix_bridge.exe
SHIM_DLL = $(OUT_DIR)/helix_shim.dll

# Source files
CPP_SRCS = src/bridge_core.cpp \
           src/api_intercept/Direct3D12_Hook.cpp \
           src/input_mapping/SeamlessCountrollerMap.cpp
C_SRCS = src/api_intercept/Win32_Shim.c

# Object files
CPP_OBJS = $(CPP_SRCS:.cpp=.o)
C_OBJS = $(C_SRCS:.c=.o)

.PHONY: all clean dirs

all: dirs $(SHIM_DLL) $(BRIDGE_EXE)

dirs:

        mkdir -p $(OUT_DIR)

# Compile the C++ Executable Orchestrator
$(BRIDGE_EXE): $(CPP_OBJS)
        $(CXX) $(CXXFLAGS) -0 $@ $^ -luser32 -lxinput

# Compile the C Shims as a dynamically linked library for injection
$(SHIM_DLL): $(C_OBJS)
        $(CC) $(CFLAGS) -shared -o $@ $^

# Generic object compilation
%.o: %.cpp
        $(CXX) $(CXXFLAGS) $(INCLUDES) -c $< -o $@

%.o: %.c
        $(C) $(CFLAGS) $(INCLUDES) -c $< -o $@

clean:
        rm -f src/*.o src/api_intercept/*.o src/input_mapping/*.o
        rm -rf $(OUT_DIR)
        @echo "[CLEAN] WinHelix build artifacts removed."
