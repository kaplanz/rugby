//
//  LCD.metal
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
half4 lcd(
    float2 position,
    SwiftUI::Layer layer,
    float2 ires,
    float2 ores
) {
    float2 sub = fract(position * ires / ores) * 6;

    // horizontal
    half4 lt = layer.sample(position + float2(-1, 0));
    half4 md = layer.sample(position + float2(+0, 0));
    half4 rt = layer.sample(position + float2(+1, 0));

    // vertical
    if (sub.y < 1) {
        // sample
        lt = mix(lt, layer.sample(position + float2(-1, -1)), .5 - sub.y / 2);
        md = mix(md, layer.sample(position + float2(+0, -1)), .5 - sub.y / 2);
        rt = mix(rt, layer.sample(position + float2(+1, -1)), .5 - sub.y / 2);
        // blend
        lt *= sub.y * SCANLINE + (1 - SCANLINE);
        md *= sub.y * SCANLINE + (1 - SCANLINE);
        rt *= sub.y * SCANLINE + (1 - SCANLINE);
    } else if (sub.y > 5) {
        // sample
        lt = mix(lt, layer.sample(position + float2(-1, +1)), (sub.y - 5) / 2);
        md = mix(md, layer.sample(position + float2(+0, +1)), (sub.y - 5) / 2);
        rt = mix(rt, layer.sample(position + float2(+1, +1)), (sub.y - 5) / 2);
        // blend
        lt *= (6 - sub.y) * SCANLINE + (1 - SCANLINE);
        md *= (6 - sub.y) * SCANLINE + (1 - SCANLINE);
        rt *= (6 - sub.y) * SCANLINE + (1 - SCANLINE);
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
