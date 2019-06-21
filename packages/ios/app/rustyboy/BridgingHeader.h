//
//  BridgingHeader.h
//  rustyboy
//
//  Created by Felix Leveille on 2019-06-20.
//  Copyright © 2019 Félix Léveillé. All rights reserved.
//

#ifndef BridgingHeader_h
#define BridgingHeader_h

#import "rustyboy.h"
#include <simd/simd.h>

struct Vertex {
    vector_float4 color;
    vector_float2 pos;
};

#endif /* BridgingHeader_h */
