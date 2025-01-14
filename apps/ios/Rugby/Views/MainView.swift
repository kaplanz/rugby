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
                .navigationTitle("Library")
                .sheet(isPresented: $manage) {
                    NavigationStack {
                        SettingsView()
                            .navigationTitle("Settings")
                            .toolbar {
                                Button("Done") {
                                    manage.toggle()
                                }
                                .bold()
                            }
                    }
                }
                .toolbar {
                    Button("Settings", systemImage: "gearshape") {
                        manage.toggle()
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
