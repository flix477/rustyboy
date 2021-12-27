#include <metal_stdlib>
#include "ShaderDefinitions.h"

using namespace metal;

struct VertexOut {
    float2 textureCoordinate;
    float4 position [[position]];
};

vertex VertexOut vertexShader(const device Vertex *vertexArray [[buffer(0)]], unsigned int vid [[vertex_id]]) {
    Vertex in = vertexArray[vid];
    VertexOut out = {
        in.textureCoordinate,
        float4(in.position.x, in.position.y, 0, 1),
    };

    return out;
}

float4 colorOutput(const float4 color, const FragmentShaderParams params) {
    if (params.darkMode) {
        return float4(1 - color.r, 1 - color.g, 1 - color.b, color.a);
    }

    return color;
}

fragment float4 fragmentShader(VertexOut in [[stage_in]],
                               texture2d<half> texture [[texture(0)]],
                               constant FragmentShaderParams &params [[buffer(0)]]) {
    const uint2 pixelSize = uint2(params.renderSize.x / params.textureSize.x,
                                  params.renderSize.y / params.textureSize.y);

    if (uint(in.position.x) % pixelSize.x == 0 || uint(in.position.y) % pixelSize.y == 0) {
        return colorOutput(float4(0, 0, 0, 1), params);
    }
    
    constexpr sampler textureSampler(mag_filter::nearest, min_filter::nearest);
    const half4 color = texture.sample(textureSampler, in.textureCoordinate);
    return colorOutput(float4(color), params);
}
