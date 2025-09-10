//
//  ShaderPicker.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-09.
//

import SwiftUI

struct ShaderPicker: View {
    @Binding var tex: Shader?

    private var frame: UIImage {
        // Load bundled unused image
        let img = UIImage(named: "unused")!
        // Recolor with this palette
        return img.cgImage.flatMap(GameIcon.redraw).map(UIImage.init(cgImage:)) ?? img
    }

    var body: some View {
        Form {
            // Preview
            Section {
                Screen(frame: frame)
                    .id(frame)
            }
            .listRowBackground(Color.clear)
            .listRowInsets(.all, 8)
            // Picker
            Picker(selection: $tex) {
                Text("None")
                    .tag(Shader?.none)
                ForEach(Shader.allCases) { tex in
                    Text(tex.description)
                        .tag(Shader?.some(tex))
                }
            } label: {
                Label("Shader", systemImage: "sparkles.tv")
            }
            .pickerStyle(.inline)
        }
        .navigationTitle("Shader")
    }
}

#Preview {
    ShaderPicker(tex: .constant(.lcd))
        .environment(Options())
}
