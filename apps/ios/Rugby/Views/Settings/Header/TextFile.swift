//
//  TextFile.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-21.
//

import MarkdownUI
import SwiftUI

extension TextFile {
    enum Kind {
        case document
        case markdown
    }
}

struct TextFile: View {
    private let path: URL
    private let kind: Kind

    init?(named path: String, kind: Kind = .document) {
        // Ensure path exists
        guard let path = Bundle.main.url(forResource: path, withExtension: nil) else {
            return nil
        }

        self.path = path
        self.kind = kind
    }

    private var text: String? {
        try? String(contentsOf: path, encoding: .utf8)
    }

    var body: some View {
        Group {
            if let text {
                switch kind {
                case .document:
                    ScrollView([.horizontal, .vertical]) {
                        Text(text)
                            .monospaced()
                            .font(.caption)
                            .textSelection(.enabled)
                    }
                case .markdown:
                    ScrollView(.vertical) {
                        Markdown(text)
                    }
                }
            } else {
                EmptyView()
            }
        }
        .padding()
        .navigationTitle(path.deletingPathExtension().lastPathComponent)
    }
}

#Preview {
    TextFile(named: "LICENSE-MIT")
}
