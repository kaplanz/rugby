//
//  SpeedPicker.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-12.
//

import SwiftUI

struct SpeedPicker: View {
    @Binding var speed: Speed

    var body: some View {
        Form {
            Text(speed.description)
        }
    }
}

#Preview {
    SpeedPicker(speed: .constant(.actual))
}
