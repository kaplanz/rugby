//
//  ShaderPicker.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-09.
//

import SwiftUI

struct ShaderPicker: View {
    @Binding var shader: Shader?

    var body: some View {
        Form {
            // Preview
            ScreenView(frame: UIImage(named: "unused"))
                .listRowBackground(Color.clear)
            // Picker
            Picker(selection: $shader) {
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
    ShaderPicker(shader: .constant(.lcd))
        .environment(Options())
}
