//
//  Screen.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-13.
//

import SwiftUI

struct ScreenView: View {
    /// Render frame.
    @State var frame: UIImage?

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            Screen(frame: frame)
            HStack(alignment: .firstTextBaseline) {
                Text("Rugby")
                    .font(.custom("Pretendo", size: 20))
                Text("Game Boy")
                    .font(.custom("Cabin", size: 36))
                    .fontWeight(.semibold)
                    .italic()
                    .textCase(.uppercase)
            }
            .baselineOffset(5)
            .foregroundStyle(.print)
        }
        .padding(6)
    }
}

#Preview {
    ScreenView()
}

struct Screen: View {
    @Environment(Options.self) private var opt

    /// Render frame.
    @State var frame: UIImage?

    /// Missing frame.
    private var empty: UIImage {
        ImageRenderer(
            content: Rectangle()
                .fill(.black)
                .frame(width: 160, height: 144)
        ).uiImage!
    }

    /// Border shape.
    private var shape: some Shape {
        .rect(cornerRadius: 4)
    }

    /// Shader function.
    private var shader: ShaderFunction? {
        opt.data.tex?.shader
    }

    var body: some View {
        GeometryReader { geo in
            Image(uiImage: frame ?? empty)
                .resizable()
                .interpolation(.none)
                .if(shader != nil) { view in
                    view.layerEffect(
                        shader!(
                            .float2(160, 144),
                            .float2(geo.size),
                        ),
                        maxSampleOffset: .zero,
                    )
                }
        }
        .aspectRatio(10 / 9, contentMode: .fit)
        .padding(6)
        .border(.black, width: 8)
        .clipShape(shape)
        .glassEffect(in: shape)
    }
}

#Preview {
    Screen()
}
