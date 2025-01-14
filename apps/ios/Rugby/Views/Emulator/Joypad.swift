//
//  Joypad.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-13.
//

import SwiftUI

struct Joypad: View {
    let part: Part

    init(part: Part = .full) {
        self.part = part
    }

    var body: some View {
        switch part {
        case .full:
            VStack(spacing: 32) {
                HStack {
                    dpad
                    Spacer()
                    game
                }
                menu
            }
        case .left:
            VStack {
                Spacer()
                dpad
                Spacer()
                menu
            }
            .padding()
        case .right:
            VStack {
                Spacer()
                game
                Spacer()
                Spacer()
            }
            .padding()
        }
    }

    var dpad: some View {
        Image("DPad")
            .padding(10)
    }

    var game: some View {
        HStack {
            Image("Game")
            Image("Game")
        }
        .rotationEffect(.degrees(-30))
    }

    var menu: some View {
        HStack {
            Image("Menu")
                .rotationEffect(.degrees(-30))
            Image("Menu")
                .rotationEffect(.degrees(-30))
        }
    }

    enum Part {
        case full
        case left
        case right
    }
}

#Preview {
    Joypad()
        .environment(GameBoy())
}
