//
//  AppIcon.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-14.
//

import SwiftUI

struct AppIcon: View {
    var body: some View {
        Image("AppIcon")
            .resizable()
            .scaledToFit()
            .frame(width: 80)
            .clipShape(.rect(cornerRadius: 18, style: .continuous))
    }
}

#Preview {
    AppIcon()
}
