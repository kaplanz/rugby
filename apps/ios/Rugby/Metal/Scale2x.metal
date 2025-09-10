//
//  Scale2x.metal
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-08.
//

#include <metal_stdlib>
#include <SwiftUI/SwiftUI_Metal.h>

using namespace metal;

[[ stitchable ]]
half4 scale2x(
    float2 position,
    SwiftUI::Layer layer,
    float2 ires,
    float2 ores
) {
    // Conversion ratio
    float2 i2o = ores / ires;
    float2 o2i = ires / ores;

    // Texel coordinates
    float2 pos = position * o2i;
    float2 sub = fract(pos) * 2;
    float2 mid = floor(pos) + 0.5;

    // Pixel coordinates
    float2 midpoint = mid * i2o;

    // Adjacent texels
    half4 B = layer.sample(midpoint + float2(+0, -1) * i2o);
    half4 D = layer.sample(midpoint + float2(-1, +0) * i2o);
    half4 E = layer.sample(midpoint + float2(+0, +0) * i2o);
    half4 F = layer.sample(midpoint + float2(+1, +0) * i2o);
    half4 H = layer.sample(midpoint + float2(+0, +1) * i2o);

    // Scaling conversion
    if (any(B != H) && any(D != F)) {
        if (sub.y < 1)
            if (sub.x < 1)
                return all(D == B) ? D : E;
            else
                return all(B == F) ? F : E;
        else
            if (sub.x < 1)
                return all(D == H) ? D : E;
            else
                return all(H == F) ? F : E;
    } else {
        return E;
    }
}
