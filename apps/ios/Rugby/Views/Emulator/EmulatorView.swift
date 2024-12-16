//
//  EmulatorView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import RugbyKit
import SwiftUI

struct EmulatorView: View {
    @State var game: Game
    @State var frame: Data?

    var body: some View {
        Text(frame?.base64EncodedString() ?? "Nothing")
            .onAppear {
                DispatchQueue.global(qos: .background).async {
                    // Initialize emulator
                    let emu = GameBoy()
                    // Insert cartridge
                    emu.insert(rom: Data(game.data))
                    // Run until emulator exits
                    while emu.ready() {
                        // Tick a single cycle
                        emu.cycle()
                        // Update video frame
                        if emu.vsync() {
                            frame = emu.frame();
                            print("HERE")
                        }
                    }
                }
            }
    }
}

#Preview {
    EmulatorView(
        game: Game(
            path: Bundle.main.url(
                forResource: "roms/games/porklike/porklike",
                withExtension: "gb"
            )!))
}
