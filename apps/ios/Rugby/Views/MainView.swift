//
//  MainView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct MainView: View {
    @Environment(GameBoy.self) private var emu
    @Environment(\.colorScheme) var colorScheme

    /// Manage application settings.
    @State private var manage = false

    var body: some View {
        @Bindable var emu = emu

        NavigationStack {
            LibraryView()
                .sheet(isPresented: $manage) {
                    NavigationStack {
                        SettingsView()
                            .toolbar {
                                Button("Done") {
                                    manage.toggle()
                                }
                                .bold()
                            }
                    }
                }
                .toolbar {
                    Button("Settings", systemImage: "gearshape") {
                        manage.toggle()
                    }
                }
        }
        .fullScreenCover(isPresented: $emu.show) {
            NavigationStack {
                EmulatorView()
            }
        }
        .tint(tint)
    }

    var tint: Color {
        // Define palette color
        let hex = emu.cfg.data.pal.data.last!
        let rgb = (
            r: Double((hex >> 16) & 0xFF) / 255,
            g: Double((hex >> 08) & 0xFF) / 255,
            b: Double((hex >> 00) & 0xFF) / 255
        )
        let pal = UIColor(red: rgb.r, green: rgb.g, blue: rgb.b, alpha: 1.0)
        // Define tinting color
        var hsl = (h: CGFloat(), s: CGFloat(), l: CGFloat())

        // Use palette for hue/saturation/brightness
        pal.getHue(&hsl.h, saturation: &hsl.s, brightness: &hsl.l, alpha: nil)

        // Update color values
        hsl.s *= colorScheme == .light ? 2.0 : 1.0
        hsl.l = colorScheme == .light ? 0.5 : 1.0

        return Color(hue: hsl.h, saturation: hsl.s, brightness: hsl.l)
    }
}

#Preview {
    MainView()
        .environment(GameBoy())
        .environment(Library())
}
