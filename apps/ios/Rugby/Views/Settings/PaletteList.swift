//
//  PaletteList.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import SwiftUI

struct PaletteList: View {
    let pal: [Palette]

    var body: some View {
        List(pal) { pal in
            PaletteView(pal: pal)
                .listRowInsets(.init(top: 0, leading: 0, bottom: 0, trailing: 0))
        }
    }
}

#Preview {
    PaletteList(pal: [.mono, .legacy])
}

struct PaletteView: View {
    let pal: Palette

    var body: some View {
        HStack(spacing: 0) {
            ForEach(pal.data, id: \.self) { color in
                Color(rgb: color)
            }
        }
        .ignoresSafeArea()
    }
}
