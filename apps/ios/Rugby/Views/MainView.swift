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
    @Environment(Failure.self) private var err

    /// Settings page.
    @State private var showSettings = false
    /// Failures page.
    @State private var showFailures = false

    /// Emulator page.
    private var showEmulator: Binding<Bool> {
        .init {
            app.active
        } set: { newValue in
            if !newValue {
                do { try app.stop() } catch { err.log(error) }
            }
        }
    }

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
                    if !err.this.isEmpty || !err.past.isEmpty {
                        ToolbarItem {
                            Button("Failures", systemImage: "exclamationmark.triangle") {
                                showFailures.toggle()
                            }
                            .buttonStyle(.borderedProminent)
                            .tint(.yellow)
                            .badge(err.this.count)
                        }
                    }
                    ToolbarItem {
                        Button("Settings", systemImage: "gearshape.fill") {
                            showSettings.toggle()
                        }
                    }
                }
        }
        .fullScreenCover(isPresented: showEmulator) {
            NavigationStack {
                EmulatorView(emu: app.emu!)
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
        .sheet(isPresented: $showFailures) {
            NavigationStack {
                FailureView()
                    .toolbar {
                        ToolbarItem(placement: .confirmationAction) {
                            Button("Done", systemImage: "checkmark", role: .confirm) {
                                showFailures.toggle()
                            }
                        }
                    }
            }
            .presentationDragIndicator(.visible)
            .presentationDetents([.medium, .large])
        }
        .sheet(isPresented: showWelcome) {
            WelcomeView()
        }
        .onChange(of: err.this) { _, newValue in
            // Stop emulation on error
            if !newValue.isEmpty {
                do { try app.stop() } catch { err.log(error) }
            }
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
