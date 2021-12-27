#ifndef ShaderDefinitions_h
#define ShaderDefinitions_h

#include <simd/simd.h>

struct Vertex {
    vector_float2 textureCoordinate;
    vector_float2 position;
};

struct FragmentShaderParams {
    vector_uint2 renderSize;
    vector_uint2 textureSize;
    bool darkMode;
};

#endif /* ShaderDefinitions_h */
