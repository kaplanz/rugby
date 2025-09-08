//
//  Screen.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-13.
//

import SwiftUI

struct ScreenView: View {
    @Environment(Options.self) private var opt

    @State var frame: UIImage?

    private var scale: ShaderFunction? {
        opt.data.tex?.scale
    }

    private var empty: UIImage {
        ImageRenderer(
            content: Rectangle()
                .fill(.black)
                .frame(width: 160, height: 144)
        ).uiImage!
    }

    private var shape: some Shape {
        .rect(cornerRadius: 4)
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            GeometryReader { geo in
                Image(uiImage: frame ?? empty)
                    .resizable()
                    .interpolation(.none)
                    .if(scale != nil) { view in
                        view.layerEffect(
                            scale!(
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
