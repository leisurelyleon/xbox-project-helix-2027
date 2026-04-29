// Hardware Geometry Culling Compute Shader
// Discards unseen geometry before it ever hits the rasterizer pipeline

struct Meshlet {
    float3 center;
    float radius;
    float3 coneApex;
    float3 coneAxis;
    float coneCutoff;
};

StructuredBuffer<Meshlet> InputMeshlets : register(t0);
RWStructuredBuffer<uint> VisibleMeshletIndices : register(u0);
RWBuffer<uint> DispatchIndirectArgs : register(u1);

cbuffer CameraData : register(b0) {
    float4x4 ViewProjectionMatrix;
    float3 CameraPosition;
};

[numthreads(64, 1, 1)]
void CSMain(uint3 DispatchThreadID : SV_DispatchThreadID)
{
    uint meshletIndex = DispatchThreadID.x;
    Meshlet m = InputMeshlets[meshletIndex];
    
    bool isVisible = true;
    
    // 1. Frustum Culling (Is it outside the camera's view?)
    float4 clipPos = mul(float4(m.center, 1.0), ViewProjectionMatrix);
    if (clipPos.z < -m.radius || clipPos.x < -clipPos.w - m.radius || clipPos.x > clipPos.w + m.radius) {
        isVisible = false;
    }
    
    // 2. Backface Culling (Is the cluster facing completely away from the camera?)
    float3 viewDir = normalize(m.coneApex - CameraPosition);
    if (dot(viewDir, m.coneAxis) >= m.coneCutoff) {
        isVisible = false;
    }
    
    // 3. Append to visible list if it survived the culling
    if (isVisible) {
        // InterlockedAdd is a thread-safe atomic operation on the GPU
        uint writeOffset;
        InterlockedAdd(DispatchIndirectArgs[0], 1, writeOffset);
        VisibleMeshletIndices[writeOffset] = meshletIndex;
    }
}
