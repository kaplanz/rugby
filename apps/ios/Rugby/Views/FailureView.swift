//
//  FailureView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-01.
//

import SwiftUI

struct FailureView: View {
    @Environment(Failure.self) private var err
    @Environment(\.dismiss) private var dismiss

    /// Error history.
    @State private var showHistory = false

    var body: some View {
        List {
            // Current
            if !err.this.isEmpty {
                Section {
                    ForEach(err.this.reversed()) { error in
                        FailureItem(item: error)
                            .swipeActions(edge: .leading, allowsFullSwipe: true) {
                                Button("Report", systemImage: "exclamationmark.bubble") {
                                    Self.report(error: error)
                                }
                                .tint(.accentColor)
                            }
                    }
                    .onDelete { offsets in
                        // Reverse offsets to correct for reversed list
                        let offsets = IndexSet(offsets.map { err.this.count - 1 - $0 })
                        withAnimation {
                            err.this.remove(atOffsets: offsets)
                        }
                    }
                }
            }
            // History
            if !err.past.isEmpty {
                Section("History", isExpanded: $showHistory) {
                    ForEach(err.past.reversed()) { error in
                        FailureItem(item: error)
                            .swipeActions(edge: .leading, allowsFullSwipe: true) {
                                Button("Report", systemImage: "exclamationmark.bubble") {
                                    Self.report(error: error)
                                }
                                .tint(.accentColor)
                            }
                    }
                    .onDelete { offsets in
                        // Reverse offsets to correct for reversed list
                        let offsets = IndexSet(offsets.map { err.past.count - 1 - $0 })
                        withAnimation {
                            err.past.remove(atOffsets: offsets)
                        }
                    }
                }
            }
            // Message
            Text(
                """
                Errors will be reported here as they occur. Not all errors are \
                considered bugs! Please consider before making a report.
                """
            )
            .font(.footnote)
            .foregroundStyle(.secondary)
            .listRowBackground(Color.clear)
            .listRowSpacing(0)
        }
        .listStyle(.sidebar)
        .navigationTitle("Errors")
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .cancellationAction) {
                Button("Clear", systemImage: "trash", role: .destructive) {
                    // Clear errors
                    err.clearAll()
                    // Dismiss view
                    dismiss()
                }
                .tint(.red)
            }
        }
        .onAppear {
            // Expand history when there's no current errors
            showHistory = err.this.isEmpty
        }
        .onDisappear {
            err.clear()
        }
    }

    static private func report(error: Failure.Error) {
        // Make mailto URL
        var mailto = URLComponents()
        mailto.scheme = "mailto"
        mailto.path = "bugs@zakhary.dev"
        mailto.queryItems = [
            URLQueryItem(
                name: "subject", value: "[\(Build.NAME)/iOS] Error Report"),
            URLQueryItem(
                name: "body",
                value: """
                    UUID: \(error.id)
                    Date: \(error.clk.formatted(.iso8601))
                    Info: \(error.msg)
                    """,
            ),
        ]
        // Open mailto URL
        guard let url = mailto.url else {
            log.error("failed to create mailto URL")
            return
        }
        UIApplication.shared.open(url)
    }

}

#Preview {
    FailureView()
        .environment(Failure())
}

private struct FailureItem: View {
    var item: Failure.Error

    var body: some View {
        Label {
            VStack(alignment: .leading) {
                Text(item.msg)
                Text(item.clk.formatted(date: .numeric, time: .standard))
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        } icon: {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundStyle(.yellow)
        }
    }
}
