//
//  SpeedPicker.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-12.
//

import SwiftUI

struct SpeedPicker: View {
    @Binding var speed: Speed

    var body: some View {
        Form {
            // Variant
            Picker(
                "Speed",
                selection: .init(
                    get: { speed.kind },
                    set: { newValue in
                        speed =
                            switch newValue {
                            case .actual:
                                .actual
                            case .ratio:
                                .ratio(Double(speed.freq ?? CLOCK) / Double(CLOCK))
                            case .clock:
                                .clock(speed.freq ?? CLOCK)
                            case .frame:
                                .frame(UInt8((speed.freq ?? CLOCK) / VIDEO))
                            case .turbo:
                                .turbo
                            }
                    },
                )
            ) {
                ForEach(Kind.allCases, id: \.self) { kind in
                    Text(kind.rawValue)
                }
            }
            .pickerStyle(.inline)
            // Options
            switch speed {
            case .ratio(let mult):
                Section {
                    Stepper(
                        speed.description,
                        value: .init(
                            get: { mult },
                            set: { speed = .ratio($0.rounded(toPlaces: 2)) },
                        ), step: 0.01)
                } header: {
                    Text("Ratio")
                } footer: {
                    Text("Speedup percentage of actual hardware frequency.")
                }
            case .clock(let freq):
                Section {
                    Stepper(
                        speed.description,
                        value: .init(
                            get: { freq },
                            set: { speed = .clock($0) },
                        ), step: 1024)
                } header: {
                    Text("Clock")
                } footer: {
                    Text("Frequency used to clock the emulator.")
                }
            case .frame(let rate):
                Section {
                    Stepper(
                        speed.description,
                        value: .init(
                            get: { rate },
                            set: { speed = .frame($0) },
                        ), step: 1)
                } header: {
                    Text("Frame")
                } footer: {
                    Text("Synchronize video output to frame rate.")
                }
            default:
                EmptyView()
            }
        }
    }

    /// Speed specifier.
    enum Kind: String, CaseIterable {
        case actual = "Actual"
        case ratio = "Ratio"
        case clock = "Clock"
        case frame = "Frame"
        case turbo = "Turbo"
    }
}

extension Speed {
    var kind: SpeedPicker.Kind {
        switch self {
        case .actual: .actual
        case .ratio(_): .ratio
        case .clock(_): .clock
        case .frame(_): .frame
        case .turbo: .turbo
        }
    }
}

#Preview {
    SpeedPicker(speed: .constant(.actual))
}
