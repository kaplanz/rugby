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
            // Custom
            NavigationLink {
                CustomPalette(pal: $pal)
            } label: {
                HStack {
                    Label {
                        Text("Custom")
                    } icon: {
                        if case .custom = pal {
                            PaletteIcon(pal: pal)
                        } else {
                            Image(systemName: "paintpalette")
                        }
                    }
                    Spacer()
                    if case .custom = pal {
                        Image(systemName: "checkmark")
                            .fontWeight(.semibold)
                            .foregroundStyle(.tint)
                    }
                }
            }
            // Picker
            Picker(selection: $pal) {
                ForEach(Palette.Name.allCases) { pal in
                    let pal = Palette.preset(named: pal)
                    Label {
                        Text(pal.description)
                    } icon: {
                        PaletteIcon(pal: pal)
                    }
                    .tag(pal)
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
            .navigationTitle("Palette")
        }
    }
}

#Preview {
    PalettePicker(pal: .constant(.default))
        .environment(Options())
}
