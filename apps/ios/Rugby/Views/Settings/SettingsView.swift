//
//  SettingsView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct SettingsView: View {
    @Environment(\.openURL) private var openURL

    @State private var pal = Palette.mono
    @State private var spd = Speed.actual

    var body: some View {
        Form {
            // Application
            Section {
                Picker("Palette", selection: $pal) {
                    ForEach(Palette.allCases) { pal in
                        Text(pal.description)
                    }
                }
                .pickerStyle(.navigationLink)
                Picker("Speed", selection: $spd) {
                    ForEach(Speed.allCases) { spd in
                        Text(spd.description)
                    }
                }
            } header: {
                Text("Application")
            }
            // About
            Section {
                Link(
                    destination: URL(
                        string: "https://github.com/kaplanz/rugby"
                    )!
                ) {
                    Label("Website", systemImage: "globe")
                }
                Menu {
                    NavigationLink {
                        LicenseView(path: "LICENSE-MIT")
                    } label: {
                        Label("MIT", systemImage: "building.columns")
                    }
                    NavigationLink {
                        LicenseView(path: "LICENSE-APACHE")
                    } label: {
                        Label("Apache-2.0", systemImage: "bird")
                    }
                } label: {
                    Label("License", systemImage: "doc.text")
                        .frame(maxWidth: .infinity, alignment: .leading)
                }
            } header: {
                Text("About")
            } footer: {
                let vers = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as! String
                Text("Version \(vers)")
            }
            Section {
                NavigationLink {
                    CreditsView()
                } label: {
                    Label("Credits", systemImage: "person.2")
                        .foregroundStyle(Color.accentColor)
                }
            } footer: {
                let date = Date.now.formatted(.dateTime.year())
                Text("Copyright &copy; \(date) Zakhary Kaplan")
            }
        }
    }
}

#Preview {
    NavigationStack {
        SettingsView()
    }
}

enum Palette: String, CaseIterable, CustomStringConvertible, Identifiable {
    case mono

    // impl CustomStringConvertible
    var description: String {
        rawValue.capitalized
    }

    // impl Identifiable
    var id: Self { self }
}

enum Speed: Float, CaseIterable, CustomStringConvertible, Identifiable {
    case half = 0.5
    case actual = 1.0
    case double = 2.0
    case turbo = 0.0
    // impl CustomStringConvertible
    var description: String {
        switch self {
        case .turbo:
            "Turbo"
        default:
            rawValue.formatted(.percent)
        }
    }

    // impl Identifiable
    var id: Self { self }
}
