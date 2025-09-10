//
//  DMG.metal
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-10.
//

#include <metal_stdlib>
#include <SwiftUI/SwiftUI_Metal.h>

using namespace metal;

constant float BLOOMING = 0.25;
constant float SCANLINE = 0.15;

[[ stitchable ]]
half4 dmg(
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

    // Scanline factor
    float mult = 1;
    // Blending (horizontal)
    if (sub.x < 1) {
        mult *= 1 + SCANLINE * (1 - sub.x);
    } else if (sub.x > 5) {
        mult *= 1 + SCANLINE * (sub.x - 5);
    }
    // Blending (vertical)
    if (sub.y < 1) {
        mult *= 1 + SCANLINE * (1 - sub.y);
    } else if (sub.y > 5) {
        mult *= 1 + SCANLINE * (sub.y - 5);
    }

    // Coordinates (main)
    pos -= float2(.5, .5);
    // Bilinear sampling
    half4 q11 = layer.sample((floor(pos) + .5) * i2o);
    half4 q12 = layer.sample((float2(floor(pos.x),  ceil(pos.y)) + .5) * i2o);
    half4 q21 = layer.sample((float2( ceil(pos.x), floor(pos.y)) + .5) * i2o);
    half4 q22 = layer.sample((ceil(pos) + .5) * i2o);
    // Interpolation (horizontal)
    float2 step = smoothstep(0, 1, fract(pos));
    half4 r1 = mix(q11, q21, step.x);
    half4 r2 = mix(q12, q22, step.x);
    // Interpolation (vertical)
    half4 main = mix(layer.sample(position) * mult, mix(r1, r2, step.y), BLOOMING);
    main.a = 1;

    // Coordinates (drop)
    pos += float2(-.6, -.8);
    // Bilinear sampling
    q11 = layer.sample((floor(pos) + .5) * i2o);
    q12 = layer.sample((float2(floor(pos.x),  ceil(pos.y)) + .5) * i2o);
    q21 = layer.sample((float2( ceil(pos.x), floor(pos.y)) + .5) * i2o);
    q22 = layer.sample((ceil(pos) + .5) * i2o);
    // Interpolation (horizontal)
    r1 = mix(q11, q21, fract(pos.x));
    r2 = mix(q12, q22, fract(pos.x));
    // Interpolation (vertical)
    half4 drop = mix(r1, r2, fract(pos.y));

    // Final mixing
    return mix(min(drop, main), main, .75);
}
