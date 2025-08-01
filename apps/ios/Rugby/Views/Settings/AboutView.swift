//
//  AboutView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import SwiftUI
import WebKit

struct AboutView: View {
    var body: some View {
        // Header
        Section {
            HStack(spacing: 20) {
                Image("AppIcon")
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
            NavigationLink {
                WebView(
                    url: URL(
                        string: "https://git.zakhary.dev/rugby",
                    ))
            } label: {
                Label("Website", systemImage: "globe")
            }
            // License
            DisclosureGroup {
                NavigationLink {
                    LicenseView(path: "LICENSE-MIT")
                } label: {
                    Label("MIT", systemImage: "building.columns")
                        .foregroundStyle(.tint)
                }
                NavigationLink {
                    LicenseView(path: "LICENSE-APACHE")
                } label: {
                    Label("Apache-2.0", systemImage: "bird")
                        .foregroundStyle(.tint)
                }
            } label: {
                Label("License", systemImage: "doc.text")
                    .foregroundStyle(.tint)
            }
            // Credit
            NavigationLink {
                EmptyView()
            } label: {
                Label("Credits", systemImage: "person.2")
                    .foregroundStyle(.tint)
            }
            .disabled(true)
        } footer: {
            Text("Copyright &copy; \(Build.DATE.formatted(.dateTime.year())) Zakhary Kaplan")
        }
    }
}

#Preview {
    NavigationView {
        List {
            AboutView()
        }
    }
}
