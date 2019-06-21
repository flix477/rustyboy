#include <metal_stdlib>
#include "ShaderDefinitions.h"

using namespace metal;

struct VertexOut {
    float4 color;
    float4 pos [[position]];
};

vertex VertexOut vertexShader(const device Vertex *vertexArray [[buffer(0)]], unsigned int vid [[vertex_id]]) {
    Vertex in = vertexArray[vid];
    VertexOut out = {
        in.color,
        float4(in.pos.x, in.pos.y, 0, 1)
    };

    return out;
}

fragment float4 fragmentShader(VertexOut interpolated [[stage_in]]) {
    return interpolated.color;
}
