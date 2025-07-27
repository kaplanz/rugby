//
//  MainView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct MainView: View {
    @Environment(GameBoy.self) private var emu

    /// Manage application settings.
    @State private var manage = false

    var body: some View {
        @Bindable var emu = emu

        NavigationStack {
            LibraryView()
                .toolbar {
                    ToolbarItem {
                        Button("Settings", systemImage: "gearshape.fill") {
                            manage.toggle()
                        }
                    }
                }
        }
        .sheet(isPresented: $manage) {
            NavigationStack {
                SettingsView()
                    .toolbar {
                        ToolbarItem(placement: .confirmationAction) {
                            Button("Done", systemImage: "checkmark", role: .confirm) {
                                manage.toggle()
                            }
                        }
                    }
            }
        }
        .fullScreenCover(isPresented: $emu.show) {
            NavigationStack {
                EmulatorView()
            }
        }
    }
}

#Preview {
    MainView()
        .environment(GameBoy())
        .environment(Library())
}
