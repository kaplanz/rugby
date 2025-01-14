//
//  RugbyApp.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import OSLog
import SwiftUI

/// Global logger.
let log = Logger()

@main
struct RugbyApp: App {
    /// Global emulator instance.
    @State private var emu = GameBoy()
    /// Global game library.
    @State private var lib = Library()

    var body: some Scene {
        WindowGroup {
            MainView()
                .environment(emu)
                .environment(lib)
        }
    }
}
