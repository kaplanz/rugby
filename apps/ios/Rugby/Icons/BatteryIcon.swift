//
//  BatteryIcon.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-10-16.
//

import GameController
import SwiftUI

struct BatteryIcon: View {
    var level: Float
    var state: GCDeviceBattery.State

    private var color: Color? {
        switch level {
        case 0.0...0.3:
            return .red
        case 0.3...0.5:
            return .yellow
        case 0.5...1.0:
            return .green
        default:
            return nil
        }
    }

    private var image: String {
        switch (state, level) {
        case (.charging, _):
            return "battery.100.bolt"
        default:
            let level = Int(25 * (4 * level).rounded(.towardZero))
            return "battery.\(level)"
        }
    }

    private var power: Bool {
        state == .charging
    }

    var body: some View {
        Image(systemName: image)
            .symbolRenderingMode(power ? .multicolor : .palette)
            .foregroundStyle(color ?? .secondary, .secondary)
    }
}

#Preview {
    let levels: [Float] = stride(from: 0.0, through: 1.0, by: 0.25).map { $0 }
    let states: [GCDeviceBattery.State] = [
        .discharging,
        .charging,
        .full,
        .unknown,
    ]

    LazyVGrid(columns: .init(
        repeating: .init(.fixed(25)),
        count: states.count)
    ) {
        ForEach(levels, id: \.self) { level in
            ForEach(states, id: \.self) { state in
                BatteryIcon(level: level, state: state)
            }
        }
    }
}
