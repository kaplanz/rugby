//
//  SpeedPicker.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-12.
//

import SwiftUI

struct SpeedPicker: View {
    @Binding var spd: Speed

    var body: some View {
        Form {
            // Variant
            Picker(
                "Speed",
                selection: .init(
                    get: { spd.kind },
                    set: { newValue in
                        spd =
                            switch newValue {
                            case .actual:
                                .actual
                            case .ratio:
                                .ratio(Double(spd.freq ?? CLOCK) / Double(CLOCK))
                            case .clock:
                                .clock(spd.freq ?? CLOCK)
                            case .frame:
                                .frame(UInt8((spd.freq ?? CLOCK) / VIDEO))
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
            switch spd {
            case .ratio(let mult):
                Section {
                    let value = Binding(
                        get: { mult },
                        set: { spd = .ratio($0.rounded(toPlaces: 2)) },
                    )
                    Stepper(spd.description, value: value, step: 0.01)
                    Slider(value: value, in: 0...3, step: 0.05)
                        .onSubmit {
                            spd = .ratio(mult)
                        }
                } header: {
                    Text("Ratio")
                } footer: {
                    Text("Speedup percentage of actual hardware frequency.")
                }
            case .clock(let freq):
                Section {
                    let value = Binding(
                        get: { freq },
                        set: { spd = .clock($0) },
                    )
                    Stepper(spd.description, value: value, step: 1024)
                    Slider(
                        value: .init(
                            get: { Float(value.wrappedValue) },
                            set: { value.wrappedValue = .init($0) },
                        ), in: 0...Float(3 * CLOCK), step: 1024)
                } header: {
                    Text("Clock")
                } footer: {
                    Text("Frequency used to clock the emulator.")
                }
            case .frame(let rate):
                Section {
                    let value = Binding(
                        get: { rate },
                        set: { spd = .frame($0) },
                    )
                    Stepper(spd.description, value: value, step: 1)
                    Slider(
                        value: .init(
                            get: { Float(value.wrappedValue) },
                            set: { value.wrappedValue = .init($0) },
                        ), in: 0...180, step: 1)
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
    SpeedPicker(spd: .constant(.actual))
}
