//
//  Screen.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-13.
//

import SwiftUI

struct Screen: View {
    @Environment(GameBoy.self) var emu

    private var image: UIImage? {
        emu.frame.flatMap(GameBoy.render(frame:))
    }

    private var empty: UIImage {
        ImageRenderer(
            content: Rectangle()
                .fill(.black)
                .frame(width: 160, height: 144)
        ).uiImage!
    }

    var body: some View {
        Image(uiImage: image ?? empty)
            .resizable()
            .interpolation(.none)
            .padding(6)
            .border(.black, width: 6)
            .clipShape(.rect(cornerRadius: 4))
            .scaledToFit()
            .padding(6)
    }
}

#Preview {
    Screen()
        .environment(GameBoy())
}
