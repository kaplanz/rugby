//
//  MarkdownView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-29.
//

import MarkdownUI
import SwiftUI

struct MarkdownView: View {
    let path: String
    var text: String? {
        guard let path = Bundle.main.path(forResource: path, ofType: nil) else {
            return nil
        }
        return try? String(contentsOfFile: path, encoding: .utf8)
    }

    var body: some View {
        ScrollView(.vertical) {
            if let text {
                Markdown(text)
            }
        }
        .padding()
        .navigationTitle(path)
    }
}
