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
            VStack(spacing: 0) {
                Image("Game")
                Text("B")
            }
            VStack(spacing: 0) {
                Image("Game")
                Text("A")
            }
        }
        .font(.custom("Orbitron", size: 16))
        .fontWeight(.heavy)
        .foregroundStyle(.brand)
        .textCase(.uppercase)
        .rotationEffect(.degrees(-30))
    }

    var menu: some View {
        HStack {
            VStack(spacing: 0) {
                Image("Menu")
                Text("Select")
            }
            .rotationEffect(.degrees(-30))
            VStack(spacing: 0) {
                Image("Menu")
                Text("Start")
            }
            .rotationEffect(.degrees(-30))
        }
        .font(.custom("Orbitron", size: 12))
        .fontWeight(.heavy)
        .foregroundStyle(.brand)
        .textCase(.uppercase)
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
