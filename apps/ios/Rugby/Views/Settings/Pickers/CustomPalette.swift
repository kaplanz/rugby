//
//  CustomPalette.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-10.
//

import SwiftUI

struct CustomPalette: View {
    @Binding var pal: Palette

    @State private var data: [Color] = .init()

    private var frame: UIImage {
        // Load bundled unused image
        let img = UIImage(named: "unused")!
        // Recolor with this palette
        return img.cgImage.flatMap(GameIcon.redraw).map(UIImage.init(cgImage:)) ?? img
    }

    var body: some View {
        Form {
            // Preview
            Section {
                Screen(frame: frame)
                    .id(frame)
            }
            .listRowBackground(Color.clear)
            .listRowInsets(.all, 8)
            // Colours
            Section {
                ForEach(data.indices, id: \.self) { idx in
                    ColorPicker(
                        data[idx].getHexString(),
                        selection: $data[idx],
                        supportsOpacity: false,
                    )
                    .monospaced()
                }
            } header: {
                Label("Custom", systemImage: "paintpalette")
            }
        }
        .onAppear {
            data = pal.data.map(Color.init(rgb:))
        }
        .onChange(of: data) { oldValue, newValue in
            guard !oldValue.isEmpty else { return }
            pal = .custom(
                color: .init(
                    c0: data[0].rgb,
                    c1: data[1].rgb,
                    c2: data[2].rgb,
                    c3: data[3].rgb,
                ))
        }
        .navigationTitle("Custom")
    }
}

#Preview {
    CustomPalette(pal: .constant(.default))
}
