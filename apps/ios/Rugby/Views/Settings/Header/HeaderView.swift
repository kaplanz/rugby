//
//  HeaderView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import SwiftUI
import WebKit

struct HeaderView: View {
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
            NavigationLink {
                List {
                    Section {
                        NavigationLink {
                            TextFile(named: "LICENSE-MIT")
                        } label: {
                            Label("MIT", systemImage: "building.columns")
                        }
                        NavigationLink {
                            TextFile(named: "LICENSE-APACHE")
                        } label: {
                            Label("Apache-2.0", systemImage: "bird")
                        }
                    } footer: {
                        Text(
                            """
                            This project is dual-licensed under both MIT License
                            and Apache License 2.0. You have permission to use
                            this code under the conditions of either license
                            pursuant to the rights granted by the chosen
                            license.
                            """
                        )
                    }
                }
                .navigationTitle("License")
            } label: {
                Label("License", systemImage: "doc.text")
            }
            // Privacy
            NavigationLink {
                TextFile(named: "PRIVACY.md", kind: .markdown)
            } label: {
                Label("Privacy", systemImage: "hand.raised")
            }
            // Credits
            NavigationLink {
                TextFile(named: "CREDITS.md", kind: .markdown)
            } label: {
                Label("Credits", systemImage: "person.bust")
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
