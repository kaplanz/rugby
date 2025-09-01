//
//  MainView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SemVer
import SwiftUI

struct MainView: View {
    @Environment(Runtime.self) private var app

    /// Emulator page.
    @State private var showEmulator = false
    /// Settings page.
    @State private var showSettings = false

    /// Welcome data.
    @AppStorage("dev.zakhary.rugby.welcome") private var welcome: String?
    /// Welcome page.
    private var showWelcome: Binding<Bool> {
        .init {
            welcome.flatMap(SemVer.Version.init).map { $0 < Build.VERSION } ?? true
        } set: { newValue in
            welcome = newValue ? nil : Build.VERSION.versionString()
        }
    }

    var body: some View {
        NavigationStack {
            LibraryView()
                .toolbar {
                    ToolbarItem {
                        Button("Settings", systemImage: "gearshape.fill") {
                            showSettings.toggle()
                        }
                    }
                }
        }
        .fullScreenCover(isPresented: $showEmulator) {
            NavigationStack {
                EmulatorView()
            }
        }
        .sheet(isPresented: $showSettings) {
            NavigationStack {
                SettingsView()
                    .toolbar {
                        ToolbarItem(placement: .confirmationAction) {
                            Button("Done", systemImage: "checkmark", role: .confirm) {
                                showSettings.toggle()
                            }
                        }
                    }
            }
        }
        .sheet(isPresented: showWelcome) {
            WelcomeView()
        }
        .onChange(of: app.game) { _, newValue in
            showEmulator = newValue != nil
        }
    }
}

#Preview {
    MainView()
        .environment(Runtime())
        .environment(Failure())
        .environment(Options())
        .environment(Library())
}
