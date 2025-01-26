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
        // Define tinting color
        let hsl = (
            h: (1 +
                atan2(
                    sqrt(3) * (rgb.g - rgb.b),
                    2 * rgb.r - rgb.g - rgb.b
                ) / (2 * .pi)
            ).truncatingRemainder(dividingBy: 1.0),
            s: (rgb.r == rgb.g) && (rgb.g == rgb.b) ? 0 : 0.6,
            l: colorScheme == .light ? 0.6 : 0.9
        )

        return Color(hue: hsl.h, saturation: hsl.s, brightness: hsl.l)
    }
}

#Preview {
    MainView()
        .environment(GameBoy())
        .environment(Library())
}
