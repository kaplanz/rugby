//
//  MainView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct MainView: View {
    @Environment(Runtime.self) private var app

    /// Presented subview.
    @State private var page: Subview?
    private enum Subview {
        case emulator
        case settings
    }

    /// Show emulator subview.
    private var showEmulator: Binding<Bool> {
        Binding {
            self.page == .emulator
        } set: { newValue in
            self.page = newValue ? .emulator : nil
        }
    }

    /// Show settings subview.
    private var showSettings: Binding<Bool> {
        Binding {
            self.page == .settings
        } set: { newValue in
            self.page = newValue ? .settings : nil
        }
    }

    var body: some View {
        NavigationStack {
            LibraryView()
                .toolbar {
                    ToolbarItem {
                        Button("Settings", systemImage: "gearshape.fill") {
                            showSettings.wrappedValue.toggle()
                        }
                    }
                }
        }
        .sheet(isPresented: showSettings) {
            NavigationStack {
                SettingsView()
                    .toolbar {
                        ToolbarItem(placement: .confirmationAction) {
                            Button("Done", systemImage: "checkmark", role: .confirm) {
                                showSettings.wrappedValue.toggle()
                            }
                        }
                    }
            }
        }
        .fullScreenCover(isPresented: showEmulator) {
            NavigationStack {
                EmulatorView()
            }
        }
        .onChange(of: app.game) { _, newValue in
            showEmulator.wrappedValue = newValue != nil
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
