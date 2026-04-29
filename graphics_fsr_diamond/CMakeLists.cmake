cmake_minimum_required(VERSION 3.24)
project(Helix_FSR_Diamond VERSION 1.0.0 LANGUAGES CXX)

# Enforce bleeding-edge C++ standards for 2027
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# ==============================================================================
# 1. C++ Orchestration Layer
# ==============================================================================

# Gather all the C++ rendering and AI orchestrator files
set(CPP_SOURCES
    src/ai_upscaling/DiamondFrameGen.cpp
    src/ai_upscaling/TensorAccelerator.cpp
    src/rendering/MicroPolygonEngine.cpp
    src/rendering/RayTracingCore.cpp
)

# Create a static library that the main Helix Kernel can link against
add_library(helix_graphics_core STATIC ${CPP_SOURCES})

# Aggressive compiler optimizations for CPU-side graphics dispatch
if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    message(STATUS "Configuring Zen 6 CPU optimizations (Clang/GCC)...")
    target_compile_options(helix_graphics_core PRIVATE -O3 -march=native -ffast-math -flto)
elseif(CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
    message(STATUS "Configuring Zen 6 CPU optimizations (MSVC)...")
    target_compile_options(helix_graphics_core PRIVATE /O2 /fp:fast /GL /arch:AVX512)
endif()

# ==============================================================================
# 2. HLSL GPU Compute Shaders (RDNA 5 / DX12 Ultimate)
# ==============================================================================

# Locate the DirectX Shader Compiler (dxc)
find_program(DXC_EXECUTABLE NAMES dxc dxc.exe)

if(NOT DXC_EXECUTABLE)
    message(WARNING "[BUILD-WARNING] DirectX Shader Compiler (dxc) not found. Shaders will not be compiled to DXIL.")
else()
    message(STATUS "Found DXC: ${DXC_EXECUTABLE}. Configuring Shader Model 6.7 builds.")

    set(HLSL_SHADERS 
        src/shaders/fsr_diamond_pass.hlsl
        src/shaders/memory_culling.hlsl
    )

    set(COMPILED_SHADERS_DIR ${CMAKE_BINARY_DIR}/compiled_shaders)
    file(MAKE_DIRECTORY ${COMPILED_SHADERS_DIR})

    # Loop through each shader and create a custom command to compile it
    foreach(SHADER_FILE ${HLSL_SHADERS})
        get_filename_component(SHADER_NAME ${SHADER_FILE} NAME_WE)
        set(OUTPUT_DXIL "${COMPILED_SHADERS_DIR}/${SHADER_NAME}.dxil")

        add_custom_command(
            OUTPUT ${OUTPUT_DXIL}
            COMMAND ${DXC_EXECUTABLE} -T cs_6_7 -E CSMain -Fo ${OUTPUT_DXIL} ${CMAKE_CURRENT_SOURCE_DIR}/${SHADER_FILE}
            DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/${SHADER_FILE}
            COMMENT "Compiling HLSL Compute Shader to RDNA 5 DXIL: ${SHADER_NAME}"
        )

        # Add to a list so we can force compilation before the C++ code runs
        list(APPEND ALL_COMPILED_SHADERS ${OUTPUT_DXIL})
    endforeach()

    # Create a custom target that forces the shaders to compile
    add_custom_target(CompileShaders ALL DEPENDS ${ALL_COMPILED_SHADERS})
    add_dependencies(helix_graphics_core CompileShaders)
endif()
