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
                // Enable
                Toggle(isOn: $cfg.aud.enable) {
                    Label {
                        Text("Volume")
                    } icon: {
                        Image(
                            systemName: "speaker.wave.3",
                            variableValue: cfg.aud.enable ? cfg.aud.volume : 0,
                        )
                    }
                }
                // Volume
                Slider(value: $cfg.aud.volume, in: 0...1)
                    .disabled(!cfg.aud.enable)
            } header: {
                Label("Speaker", systemImage: "hifispeaker")
            }
            Section {
            } header: {
                Label("Mixing", systemImage: "hifireceiver")
            }
            .hidden()
            Section {
                Stepper(value: $cfg.aud.sample.value, in: 8000...96000, step: 100) {
                    Text(
                        cfg.aud.sample
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
                    value: $cfg.aud.sample.value,
                    in: 8000...96000,
                    step: 100,
                )
                .onSubmit {
                    cfg.aud.sample.value.round()
                }
            } header: {
                Label("Sample Rate", systemImage: "waveform")
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
