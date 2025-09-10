//
//  WelcomePal.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-14.
//

import SwiftUI

struct WelcomePal: View {
    @Environment(Options.self) private var opt

    var body: some View {
        @Bindable var cfg = opt.data
        PalettePicker(pal: $cfg.pal)
    }
}

#Preview {
    WelcomePal()
        .environment(Options())
}
