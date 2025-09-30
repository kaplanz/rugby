//
//  PlaybackSettings.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-15.
//

import SwiftUI

struct PlaybackSettings: View {
    @Environment(Options.self) private var opt

    var body: some View {
        @Bindable var cfg = opt.data

        Form {
            // Speed
            Section {
                // Reverse
                NavigationLink {
                    SpeedPicker(spd: $cfg.spd.rev)
                        .navigationTitle("Reverse")
                } label: {
                    HStack {
                        Label("Reverse", systemImage: "backward.circle")
                        Spacer()
                        Text(cfg.spd.rev.description)
                            .foregroundStyle(.secondary)
                    }
                }
                // Forward
                NavigationLink {
                    SpeedPicker(spd: $cfg.spd.fwd)
                        .navigationTitle("Forward")
                } label: {
                    HStack {
                        Label("Forward", systemImage: "forward.circle")
                        Spacer()
                        Text(cfg.spd.fwd.description)
                            .foregroundStyle(.secondary)
                    }
                }
            } header: {
                Label(
                    "Speed",
                    systemImage: "gauge.open.with.lines.needle.67percent.and.arrowtriangle")
            } footer: {
                Text(
                    """
                    Change the speedup factor used for fast-forward and reverse.
                    """
                )
            }

            // HUD
            Section {
                Toggle("Enable HUD", systemImage: "info.windshield", isOn: $cfg.hud)
            } footer: {
                Text(
                    """
                    When enabled, the heads-up display (HUD) will show the \
                    frame rate during emulation.
                    """
                )
            }

        }
        .navigationTitle("Playback")
    }
}

#Preview {
    PlaybackSettings()
        .environment(Options())
}
