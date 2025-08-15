//
//  WelcomeEnd.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-14.
//

import SwiftUI

struct WelcomeEnd: View {
    var body: some View {
        Text("Welcome!")
            .font(.largeTitle)
            .fontWeight(.heavy)
            .foregroundStyle(Color.accentColor.gradient)
            .ignoresSafeArea()
    }
}

#Preview {
    WelcomeEnd()
}
