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
    float2 sub = fract(position * ires / ores) * 3;

    half4 A = layer.sample(position + float2(-1, -1) * ores / ires);
    half4 B = layer.sample(position + float2(+0, -1) * ores / ires);
    half4 C = layer.sample(position + float2(+1, -1) * ores / ires);
    half4 D = layer.sample(position + float2(-1, +0) * ores / ires);
    half4 E = layer.sample(position + float2(+0, +0) * ores / ires);
    half4 F = layer.sample(position + float2(+1, +0) * ores / ires);
    half4 G = layer.sample(position + float2(-1, +1) * ores / ires);
    half4 H = layer.sample(position + float2(+0, +1) * ores / ires);
    half4 I = layer.sample(position + float2(+1, +1) * ores / ires);

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
