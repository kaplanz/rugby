//
//  Scale3x.metal
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-08.
//

#include <metal_stdlib>
#include <SwiftUI/SwiftUI_Metal.h>

using namespace metal;

[[ stitchable ]]
half4 scale3x(
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
    float2 sub = fract(pos) * 3;
    float2 mid = floor(pos) + 0.5;

    // Pixel coordinates
    float2 midpoint = mid * i2o;

    // Adjacent texels
    half4 A = layer.sample(midpoint + float2(-1, -1) * i2o);
    half4 B = layer.sample(midpoint + float2(+0, -1) * i2o);
    half4 C = layer.sample(midpoint + float2(+1, -1) * i2o);
    half4 D = layer.sample(midpoint + float2(-1, +0) * i2o);
    half4 E = layer.sample(midpoint + float2(+0, +0) * i2o);
    half4 F = layer.sample(midpoint + float2(+1, +0) * i2o);
    half4 G = layer.sample(midpoint + float2(-1, +1) * i2o);
    half4 H = layer.sample(midpoint + float2(+0, +1) * i2o);
    half4 I = layer.sample(midpoint + float2(+1, +1) * i2o);

    // Scaling conversion
    if (any(B != H) && any(D != F)) {
        if (sub.y < 1)
            if (sub.x < 1)
                return all(D == B) ? D : E;
            else if (sub.x < 2)
                return (all(D == B) && any(E != C)) || (all(B == F) && any(E != A)) ? B : E;
            else
                return all(B == F) ? F : E;
        else if (sub.y < 2)
            if (sub.x < 1)
                return (all(D == B) && any(E != G)) || (all(D == H) && any(E != A)) ? D : E;
            else if (sub.x < 2)
                return E;
            else
                return (all(B == F) && any(E != I)) || (all(H == F) && any(E != C)) ? F : E;
        else
            if (sub.x < 1)
                return all(D == H) ? D : E;
            else if (sub.x < 2)
                return (all(D == H) && any(E != I)) || (all(H == F) && any(E != G)) ? H : E;
            else
                return all(H == F) ? F : E;
    } else {
        return E;
    }
}
