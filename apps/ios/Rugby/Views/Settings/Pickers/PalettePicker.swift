//
//  PalettePicker.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-09.
//

import SwiftUI

struct PalettePicker: View {
    @Binding var pal: Palette

    @State private var showInfo = false

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
            // Picker
            Picker(selection: $pal) {
                ForEach(Palette.allCases) { pal in
                    Label {
                        Text(pal.description)
                    } icon: {
                        PaletteIcon(pal: pal)
                    }
                }
            } label: {
                HStack {
                    Label("Palette", systemImage: "swatchpalette")
                    Spacer()
                    Button("Info", systemImage: "info.circle") {
                        showInfo = true
                    }
                    .labelStyle(.iconOnly)
                }
                .confirmationDialog("Choose a palette", isPresented: $showInfo) {
                } message: {
                    Text(
                        """
                        Your palette selection will also be used as an accent for \
                        within this app.
                        """
                    )
                }
            }
            .pickerStyle(.inline)
        }
    }
}

#Preview {
    PalettePicker(pal: .constant(.demichrome))
        .environment(Options())
}
