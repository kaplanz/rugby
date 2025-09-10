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
            ScreenView(frame: frame)
                .listRowBackground(Color.clear)
                .id(frame)
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
    }
}

#Preview {
    ShaderPicker(tex: .constant(.lcd))
        .environment(Options())
}
