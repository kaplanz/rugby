//
//  LicenseView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-21.
//

import SwiftUI

struct LicenseView: View {
    let path: String

    private var file: License {
        License(path: path)
    }

    var body: some View {
        ScrollView([.horizontal, .vertical]) {
            Text(file.text)
                .monospaced()
                .font(.caption)
                .textSelection(.enabled)
        }
        .padding()
    }
}

private struct License {
    let path: String

    var text: String {
        let path = Bundle.main.path(forResource: path, ofType: nil)!
        return try! String(contentsOfFile: path, encoding: .utf8)
    }
}

#Preview {
    LicenseView(path: "LICENSE-MIT")
}
