//
//  PaletteIcon.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import SwiftUI

struct PaletteIcon: View {
    let pal: Palette

    var body: some View {
        LazyHGrid(
            rows: [
                .init(spacing: 2),
                .init(spacing: 2),
            ],
            spacing: 2
        ) {
            ForEach(pal.data, id: \.self) { color in
                Color(rgb: color)
                    .clipShape(.rect(cornerRadius: 2))
            }
        }
        .padding(4)
        .background(Color(rgb: pal.data.last!).opacity(0.2))
        .frame(width: 30, height: 30)
        .aspectRatio(1.0, contentMode: .fit)
        .clipShape(.rect(cornerRadius: 4))
    }
}

#Preview {
    PaletteIcon(pal: .mono)
}
