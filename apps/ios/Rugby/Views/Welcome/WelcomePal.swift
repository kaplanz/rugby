//
//  WelcomePal.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-14.
//

import SwiftUI

struct WelcomePal: View {
    @Environment(Options.self) private var opt
    @Environment(\.dismiss) private var dismiss

    @State private var showInfo = false

    var body: some View {
        @Bindable var cfg = opt.data

        Form {
            Picker(selection: $cfg.pal) {
                ForEach(Palette.allCases) { pal in
                    Label {
                        Text(pal.description)
                    } icon: {
                        PaletteIcon(pal: pal)
                    }
                }
            } label: {
                HStack {
                    Label("Palette", systemImage: "swatchpalette")
                    Spacer()
                    Button("Info", systemImage: "info.circle") {
                        showInfo = true
                    }
                    .labelStyle(.iconOnly)
                }
                .confirmationDialog("Choose a palette", isPresented: $showInfo) {
                } message: {
                    Text(
                        """
                        Your palette selection will also be used as an accent for \
                        within this app.
                        """
                    )
                }
            }
            .pickerStyle(.inline)
        }
    }
}

#Preview {
    WelcomePal()
        .environment(Options())
}
