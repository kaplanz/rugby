//
//  ControlsSettings.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-15.
//

import GameController
import RugbyKit
import SwiftUI

struct ControlsSettings: View {
    @Environment(Options.self) private var opt
    @Environment(Gamepad.self) private var pad

    var body: some View {
        Form {
            // Gamepad
            Section {
                if pad.list.isEmpty {
                    Text("None")
                        .foregroundStyle(.secondary)
                } else {
                    ForEach(pad.list, id: \.self) { gamepad in
                        HStack {
                            // About
                            VStack(alignment: .leading) {
                                // Name
                                Text(gamepad.vendorName ?? "Unknown")
                                HStack(alignment: .center, spacing: 4) {
                                    // Product
                                    Text(gamepad.productCategory)
                                    // Battery
                                    if let battery = gamepad.battery {
                                        Text("-")
                                        HStack(spacing: 2) {
                                            Battery(battery: battery)
                                                .font(.body)
                                            Text(
                                                battery.batteryLevel
                                                    .formatted(
                                                        .percent
                                                            .precision(
                                                                .fractionLength(0)
                                                            )
                                                    )
                                            )
                                        }
                                    }
                                }
                                .foregroundStyle(.secondary)
                                .font(.caption)
                            }
                            Spacer()
                            // Current
                            if pad.main == gamepad {
                                Image(systemName: "checkmark")
                                    .foregroundStyle(.tint)
                                    .bold()
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
            // Mapping
            if pad.main != nil {
                Section {
                    ForEach(RugbyKit.Button.allCases, id: \.self) { button in
                        Label(
                            String(describing: button).capitalized,
                            systemImage: button.systemImage,
                        )
                        .foregroundStyle(.primary)
                    }
                } header: {
                    Label("Input Mapping", systemImage: "button.horizontal.top.press")
                } footer: {
                    Text(
                        """
                        Press a button to see the corresponding input within \
                        the app.
                        """
                    )
                }
            }
        }
        .navigationTitle("Controls")
    }
}

#Preview {
    ControlsSettings()
        .environment(Options())
}

extension RugbyKit.Button {
    public var systemImage: String {
        switch self {
        case .a:
            "a.circle"
        case .b:
            "b.circle"
        case .start:
            "plus.diamond"
        case .select:
            "minus.diamond"
        case .up:
            "dpad.up.filled"
        case .down:
            "dpad.down.filled"
        case .left:
            "dpad.left.filled"
        case .right:
            "dpad.right.filled"
        }
    }
}

extension RugbyKit.Button: @retroactive CaseIterable {
    public static var allCases: [Self] {
        [
            .a,
            .b,
            .start,
            .select,
            .up,
            .down,
            .left,
            .right,
        ]
    }
}
