//
//  AppHeader.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import SwiftUI

struct AppHeader: View {
    var body: some View {
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
            .disabled(true)
        } footer: {
            Text("Copyright &copy; \(Build.DATE.formatted(.dateTime.year())) Zakhary Kaplan")
        }
    }
}

#Preview {
    List {
        AppHeader()
    }
}
