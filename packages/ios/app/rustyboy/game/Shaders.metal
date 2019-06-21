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
        float4(in.position.x, in.position.y, 0, 1)
    };

    return out;
}

fragment float4 fragmentShader(VertexOut in [[stage_in]], texture2d<half> texture [[texture(0)]]) {
    constexpr sampler textureSampler (mag_filter::linear, min_filter::linear);
    const half4 color = texture.sample(textureSampler, in.textureCoordinate);
    return float4(color);
}
