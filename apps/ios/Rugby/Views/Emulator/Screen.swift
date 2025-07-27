//
//  Screen.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-13.
//

import SwiftUI

struct Screen: View {
    @Environment(GameBoy.self) var emu

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
            Image(uiImage: emu.image ?? empty)
                .resizable()
                .interpolation(.none)
                .padding(6)
                .border(.black, width: 8)
                .clipShape(shape)
                .glassEffect(in: shape)
                .scaledToFit()
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
            .foregroundStyle(.text)
        }
        .padding(6)
    }
}

#Preview {
    Screen()
        .environment(GameBoy())
}
