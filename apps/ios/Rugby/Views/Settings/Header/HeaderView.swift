//
//  HeaderView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import SwiftUI
import WebKit

struct HeaderView: View {
    @State private var openLicense = false

    var body: some View {
        // Header
        Section {
            HStack(spacing: 20) {
                AppIcon()
                VStack(alignment: .leading) {
                    Text(Build.NAME)
                        .bold()
                        .font(.title)
                        .fontDesign(.rounded)
                    Text("Version \(Build.VERSION.description)")
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
            Button {
                openLicense.toggle()
            } label: {
                Label("License", systemImage: "doc.text")
                    .foregroundStyle(.tint)
            }
            .confirmationDialog("Choose License", isPresented: $openLicense) {
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
            }
            // Credit
            NavigationLink {
                MarkdownView(path: "CREDITS.md")
            } label: {
                Label("Credits", systemImage: "person.2.shield")
                    .foregroundStyle(.tint)
            }
        } footer: {
            Text("Copyright &copy; \(Build.DATE.formatted(.dateTime.year())) Zakhary Kaplan")
        }
    }
}

#Preview {
    NavigationView {
        List {
            HeaderView()
        }
    }
}
