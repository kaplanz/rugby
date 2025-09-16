//
//  VideoSettings.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-15.
//

import SwiftUI

struct VideoSettings: View {
    @Environment(Options.self) private var opt

    private var frame: UIImage {
        // Load bundled unused image
        let img = UIImage(named: "unused")!
        // Recolor with this palette
        return img.cgImage.flatMap(GameIcon.redraw).map(UIImage.init(cgImage:)) ?? img
    }

    var body: some View {
        @Bindable var cfg = opt.data

        Form {
            // Preview
            Section {
                Screen(frame: frame)
                    .id(frame)
            }
            .listRowBackground(Color.clear)
            .listRowInsets(.all, 8)
            // Shader
            Section {
                NavigationLink {
                    ShaderPicker(tex: $cfg.tex)
                } label: {
                    HStack {
                        Label("Shader", systemImage: "sparkles")
                        Spacer()
                        Text(cfg.tex?.description ?? "None")
                            .foregroundStyle(.secondary)
                    }
                }
            } footer: {
                Text(
                    """
                    Shaders allow you to add effects to the emulated display.
                    """
                )
            }
            // Palette
            Section {
                NavigationLink {
                    PalettePicker(pal: $cfg.pal)
                } label: {
                    HStack {
                        Label("Palette", systemImage: "swatchpalette")
                        Spacer()
                        Label {
                            Text(cfg.pal.description)
                        } icon: {
                            PaletteIcon(pal: cfg.pal)
                        }
                        .foregroundStyle(.secondary)
                    }
                }
            } footer: {
                Text(
                    """
                    Select a preset or customize a palette to set the colours \
                    used for original Game Boy (DMG) games.

                    Your selected palette will also be used as an accent \
                    throughout the app.
                    """
                )
            }
        }
        .navigationTitle("Video")
    }
}

#Preview {
    VideoSettings()
        .environment(Options())
}
