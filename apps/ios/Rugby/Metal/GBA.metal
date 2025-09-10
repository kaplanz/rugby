//
//  GBA.metal
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-08.
//

#include <metal_stdlib>
#include <SwiftUI/SwiftUI_Metal.h>

using namespace metal;

constant float COLOR_LO = 0.8;
constant float COLOR_HI = 1.0;
constant float SCANLINE = 0.1;

[[ stitchable ]]
half4 gba(
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
    float2 sub = fract(pos) * 6;
    float2 mid = floor(pos) + 0.5;

    // Pixel coordinates
    float2 midpoint = mid * i2o;

    // Adjacent texels (horizontal)
    half4 lt = layer.sample(midpoint + float2(-1, 0) * i2o);
    half4 md = layer.sample(midpoint + float2(+0, 0) * i2o);
    half4 rt = layer.sample(midpoint + float2(+1, 0) * i2o);

    // Adjacent texels (vertical)
    if (sub.y < 1) {
        // sample
        lt = mix(lt, layer.sample(midpoint + float2(-1, -1) * i2o), .5 - sub.y / 2);
        md = mix(md, layer.sample(midpoint + float2(+0, -1) * i2o), .5 - sub.y / 2);
        rt = mix(rt, layer.sample(midpoint + float2(+1, -1) * i2o), .5 - sub.y / 2);
        // blend
        lt *= (1 - SCANLINE) + SCANLINE * sub.y;
        md *= (1 - SCANLINE) + SCANLINE * sub.y;
        rt *= (1 - SCANLINE) + SCANLINE * sub.y;
    } else if (sub.y > 5) {
        // sample
        lt = mix(lt, layer.sample(midpoint + float2(-1, +1) * i2o), (sub.y - 5) / 2);
        md = mix(md, layer.sample(midpoint + float2(+0, +1) * i2o), (sub.y - 5) / 2);
        rt = mix(rt, layer.sample(midpoint + float2(+1, +1) * i2o), (sub.y - 5) / 2);
        // blend
        lt *= (1 - SCANLINE) + SCANLINE * (6 - sub.y);
        md *= (1 - SCANLINE) + SCANLINE * (6 - sub.y);
        rt *= (1 - SCANLINE) + SCANLINE * (6 - sub.y);
    }

    half4 ml = mix(lt, md, .5);
    half4 mr = mix(rt, md, .5);

    if (sub.x < 1) {
        half4 a = half4(COLOR_HI * md.r, COLOR_LO * md.g, COLOR_HI * lt.b, 1);
        half4 b = half4(COLOR_HI * md.r, COLOR_LO * md.g, COLOR_LO * lt.b, 1);
        return mix(a, b, sub.x);
    } else if (sub.x < 2) {
        half4 a = half4(COLOR_HI * md.r, COLOR_LO * md.g, COLOR_LO * lt.b, 1);
        half4 b = half4(COLOR_HI * md.r, COLOR_HI * md.g, COLOR_LO * ml.b, 1);
        return mix(a, b, sub.x - 1);
    } else if (sub.x < 3) {
        half4 a = half4(COLOR_HI * md.r, COLOR_HI * md.g, COLOR_LO * ml.b, 1);
        half4 b = half4(COLOR_LO * mr.r, COLOR_HI * md.g, COLOR_LO * md.b, 1);
        return mix(a, b, sub.x - 2);
    } else if (sub.x < 4) {
        half4 a = half4(COLOR_LO * mr.r, COLOR_HI * md.g, COLOR_LO * md.b, 1);
        half4 b = half4(COLOR_LO * rt.r, COLOR_HI * md.g, COLOR_HI * md.b, 1);
        return mix(a, b, sub.x - 3);
    } else if (sub.x < 5) {
        half4 a = half4(COLOR_LO * rt.r, COLOR_HI * md.g, COLOR_HI * md.b, 1);
        half4 b = half4(COLOR_LO * rt.r, COLOR_LO * mr.g, COLOR_HI * md.b, 1);
        return mix(a, b, sub.x - 4);
    } else {
        half4 a = half4(COLOR_LO * rt.r, COLOR_LO * mr.g, COLOR_HI * md.b, 1);
        half4 b = half4(COLOR_HI * rt.r, COLOR_LO * rt.g, COLOR_HI * md.b, 1);
        return mix(a, b, sub.x - 5);
    }
}
