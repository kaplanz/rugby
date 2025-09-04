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
                    }
                    .onDelete { offsets in
                        err.this.remove(atOffsets: offsets)
                    }
                }
            }
            // History
            if !err.past.isEmpty {
                Section("History", isExpanded: $showHistory) {
                    ForEach(err.past.reversed()) { error in
                        FailureItem(item: error)
                    }
                    .onDelete { offsets in
                        err.past.remove(atOffsets: offsets)
                    }
                }
            }
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
