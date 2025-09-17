//
//  AudioSettings.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-15.
//

import SwiftUI

struct AudioSettings: View {
    @Environment(Options.self) private var opt

    var body: some View {
        @Bindable var cfg = opt.data

        Form {
            Section {
                Stepper(value: $cfg.aud.value, in: 8000...96000, step: 100) {
                    Text(
                        cfg.aud
                            .converted(to: .kilohertz)
                            .formatted(
                                .measurement(
                                    width: .abbreviated,
                                    numberFormatStyle:
                                        .number.precision(.fractionLength(1))
                                ))
                    )
                }
                Slider(
                    value: $cfg.aud.value,
                    in: 8000...96000,
                    step: 100,
                )
                .onSubmit {
                    cfg.aud.value.round()
                }
            } header: {
                Label("Sample", systemImage: "dial.high")
            } footer: {
                Text(
                    """
                    Defines the sample rate to use for audio output.

                    Must be in the range of 8 kHz to 96 kHz. Unless you have a \
                    specific use case, there is no reason to change the \
                    default value.
                    """
                )
            }
            .disabled(true)
        }
        .navigationTitle("Audio")
    }
}

#Preview {
    AudioSettings()
        .environment(Options())
}
