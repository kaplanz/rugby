//
//  ControlsSettings.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-15.
//

import GameController
import SwiftUI

struct ControlsSettings: View {
    @Environment(Options.self) private var opt

    var body: some View {
        @Bindable var cfg = opt.data

        Form {
            Section {
                let controllers = GCController.controllers()
                if controllers.isEmpty {
                    Text("None")
                        .foregroundStyle(.secondary)
                } else {
                    ForEach(controllers, id: \.self) { gamepad in
                        HStack {
                            Text(gamepad.vendorName ?? "Unknown")
                            Spacer()
                            if GCController.current == gamepad {
                                Image(systemName: "checkmark")
                                    .foregroundStyle(.tint)
                            }
                        }
                    }
                }
            } header: {
                Label("Game Controllers", systemImage: "gamecontroller")
            } footer: {
                Text(
                    """
                    You can connect and manage controllers for your device in \
                    Settings > General > Game Controller.
                    """
                )
            }
        }
        .navigationTitle("Controls")
    }
}

#Preview {
    ControlsSettings()
        .environment(Options())
}
