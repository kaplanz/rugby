//
//  Battery.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-10-16.
//

import GameController
import SwiftUI

struct Battery: View {
    var battery: GCDeviceBattery

    private var color: Color? {
        let state = battery.batteryState
        let level = battery.batteryLevel
        switch (state, level) {
        case (.charging, _):
            return .green
        case (_, 0...0.2):
            return .red
        default:
            return nil
        }
    }

    private var image: String {
        let state = battery.batteryState
        let level = battery.batteryLevel
        switch (state, level) {
        case (.charging, _):
            return "battery.100.bolt"
        default:
            let level = Int(25 * (4 * level).rounded(.towardZero))
            return "battery.\(level)"
        }
    }

    var body: some View {
        Image(systemName: image)
            .symbolRenderingMode(.palette)
            .foregroundStyle(color ?? .secondary, .secondary)
    }
}
