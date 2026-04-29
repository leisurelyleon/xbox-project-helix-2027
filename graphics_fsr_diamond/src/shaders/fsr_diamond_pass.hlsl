// FSR Diamond - Neural Upscaling Compute Shader
// Target: RDNA 5 / DirectX 12 Ultimate (Shader Model 6.7+)

Texture2D<float4> InputColor : register(t0);
Texture2D<float2> InputMotionVectors : register(t1);
Texture2D<float>  InputDepth : register(t2);

// The NPU output tensor containing the AI's "guess" for the missing pixels
Texture2D<float4> NeuralTensorFeatures : register(t3);

RWTexture2D<float4> OutputColor : register(u0);

// Runs in 8x8 blocks of pixels simultaneously
[numthreads(8, 8, 1)]
void CSMain(uint3 DispatchThreadID : SV_DispatchThreadID)
{
    uint2 pixelCoord = DispatchThreadID.xy;
    
    // 1. Fetch current pixel data
    float4 baseColor = InputColor.Load(int3(pixelCoord / 2, 0)); // Assuming 2x upscaling
    float2 motion = InputMotionVectors.Load(int3(pixelCoord / 2, 0));
    float depth = InputDepth.Load(int3(pixelCoord / 2, 0));
    
    // 2. Reproject history
    // Calculate where this pixel was in the previous frame
    int2 previousPixelCoord = pixelCoord - int2(motion * 3840.0); // Assuming 4K target
    
    // 3. Apply Neural Enhancement
    // Blend the raw low-res pixel with the AI-generated high-frequency details
    float4 aiDetails = NeuralTensorFeatures.Load(int3(pixelCoord, 0));
    
    float4 finalColor = lerp(baseColor, aiDetails, 0.5f); // 50% blend for simulation
    
    // 4. Output the final 4K pixel
    OutputColor[pixelCoord] = finalColor;
}
