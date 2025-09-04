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

    var body: some View {
        List {
            // Current
            if !err.this.isEmpty {
                Section {
                    ForEach(err.this.enumerated().reversed(), id: \.offset) { _, error in
                        FailureItem(item: error)
                    }
                }
            }
            // History
            if !err.past.isEmpty {
                Section("History") {
                    ForEach(err.past.enumerated().reversed(), id: \.offset) { _, error in
                        FailureItem(item: error)
                    }
                }
            }
        }
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
