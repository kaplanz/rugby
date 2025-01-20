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
            // Header
            Section {
                HStack(spacing: 20) {
                    Image(uiImage: UIImage(named: "AppIcon60x60")!)
                        .resizable()
                        .scaledToFit()
                        .frame(width: 80)
                        .clipShape(.rect(cornerRadius: 18, style: .continuous))
                    VStack(alignment: .leading) {
                        Text(Build.NAME)
                            .bold()
                            .font(.title)
                        Text("Version \(Build.VERSION)")
                            .foregroundStyle(.secondary)
                    }
                }
            }
            .listRowBackground(Color.clear)
            .listSectionSpacing(10)
            // About
            Section {
                // Website
                Link(
                    destination: URL(
                        string: "https://github.com/kaplanz/rugby"
                    )!
                ) {
                    Label("Website", systemImage: "globe")
                }
                // License
                DisclosureGroup {
                    NavigationLink {
                        LicenseView(path: "LICENSE-MIT")
                    } label: {
                        Label("MIT", systemImage: "building.columns")
                            .foregroundStyle(Color.accentColor)
                    }
                    NavigationLink {
                        LicenseView(path: "LICENSE-APACHE")
                    } label: {
                        Label("Apache-2.0", systemImage: "bird")
                            .foregroundStyle(Color.accentColor)
                    }
                } label: {
                    Label("License", systemImage: "doc.text")
                        .foregroundStyle(Color.accentColor)
                }
                // Credit
                NavigationLink {
                    CreditsView()
                } label: {
                    Label("Credits", systemImage: "person.2")
                        .foregroundStyle(Color.accentColor)
                }
            } footer: {
                Text("Copyright &copy; \(Build.DATE.formatted(.dateTime.year())) Zakhary Kaplan")
            }
            // Application
            Section("Application") {
                // Palette
                Picker("Palette", selection: $pal) {
                    ForEach(Palette.allCases) { pal in
                        Text(pal.description)
                    }
                }
                .pickerStyle(.navigationLink)
                // Speed
                Picker("Speed", selection: $spd) {
                    ForEach(Speed.allCases) { spd in
                        Text(spd.description)
                    }
                }
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
