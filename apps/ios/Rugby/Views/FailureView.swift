//
//  FailureView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-01.
//

import SwiftUI

struct FailureView: View {
    @Environment(Failure.self) private var err

    var body: some View {
        List {
            // Current
            if let error = err.this {
                Section {
                    FailureItem(item: error)
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
                    withAnimation { err.clearAll() }
                }
                .tint(.red)
            }
            ToolbarItem(placement: .confirmationAction) {
                Button("Done", systemImage: "checkmark", role: .confirm) {
                    withAnimation { err.clear() }
                }
            }
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
