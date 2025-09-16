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
            EmptyView()
        }
        .navigationTitle("Audio")
    }
}

#Preview {
    AudioSettings()
        .environment(Options())
}
