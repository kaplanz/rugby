//
//  Joypad.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-13.
//

import RugbyKit
import SwiftUI

struct Joypad: View {
    /// Region of the joypad to render.
    let part: Region

    init(part: Region = .full) {
        self.part = part
    }

    /// Region of the joypad.
    enum Region {
        case full
        case left
        case right
    }

    var body: some View {
        Group {
            switch part {
            case .full:
                VStack(spacing: 32) {
                    HStack {
                        DPadInput()
                        Spacer()
                        GameInput()
                    }
                    MenuInput()
                }
            case .left:
                VStack {
                    Spacer()
                    DPadInput()
                    Spacer()
                    MenuInput()
                }
                .padding()
            case .right:
                VStack {
                    Spacer()
                    GameInput()
                    Spacer()
                    Spacer()
                }
                .padding()
            }
        }
    }
}

#Preview {
    Joypad()
        .environment(GameBoy())
}

private struct DPadInput: View {
    @Environment(GameBoy.self) private var emu

    /// Location within the D-pad's touch area.
    @State private var touch: (x: Float, y: Float)?

    /// Pressed D-pad inputs.
    private var input: Set<RugbyKit.Button> {
        // Ensure user is touching the D-pad
        guard let place = touch else { return [] }

        // Construct input values
        var input = Set<RugbyKit.Button>()

        // Determine x-axis
        switch place.x {
        case 0...0.33:
            input.insert(RugbyKit.Button.left)
        case 0.66...1:
            input.insert(RugbyKit.Button.right)
        default: break
        }

        // Determine y-axis
        switch place.y {
        case 0...0.33:
            input.insert(RugbyKit.Button.up)
        case 0.66...1:
            input.insert(RugbyKit.Button.down)
        default: break
        }

        return input
    }

    var body: some View {
        Group {
            Image("DPad")
                .padding(10)
                .overlay {
                    DPadTouchArea(press: $touch)
                }
                .overlay {
                    let shadow = Image("DPadShadow")
                    if input.contains(.up) {
                        shadow
                            .rotationEffect(.degrees(0))
                    }
                    if input.contains(.right) {
                        shadow
                            .rotationEffect(.degrees(90))
                    }
                    if input.contains(.down) {
                        shadow
                            .rotationEffect(.degrees(180))
                    }
                    if input.contains(.left) {
                        shadow
                            .rotationEffect(.degrees(270))
                    }
                }
        }
        .sensoryFeedback(.impact, trigger: input)
        .onChange(of: input) { prev, next in
            for button in next.subtracting(prev) {
                emu.input(button, pressed: true)
            }
            for button in prev.subtracting(next) {
                emu.input(button, pressed: false)
            }
        }
    }
}

private struct DPadTouchArea: View {
    /// Visibility of the touchable area.
    let visible: Bool = false

    /// Relative coordinates of a touch event.
    ///
    /// Values for `x` and `y` must be between 0 and 1.
    @Binding var press: (x: Float, y: Float)?

    var body: some View {
        GeometryReader { geo in
            Circle()
                .opacity(visible ? 0.1 : 0)
                .contentShape(Circle())
                .gesture(
                    DragGesture(minimumDistance: 0)
                        .onChanged { gesture in
                            press = (
                                x: Float(gesture.location.x / geo.size.width),
                                y: Float(gesture.location.y / geo.size.height)
                            )
                        }
                        .onEnded { gesture in
                            press = nil
                        }
                )
        }
    }
}

private struct GameInput: View {
    var body: some View {
        HStack {
            GameButton(role: .b)
            GameButton(role: .a)
        }
        .rotationEffect(.degrees(-30))
    }
}

private struct GameButton: View {
    @Environment(GameBoy.self) private var emu

    let role: RugbyKit.Button

    @State private var isPressed = false

    var body: some View {
        VStack(spacing: 0) {
            Image("Game")
                .overlay {
                    if isPressed {
                        Image("GameShadow")
                    }
                }
            Text(String(describing: role))
                .font(.custom("Orbitron", size: 16))
                .fontWeight(.heavy)
                .foregroundStyle(.brand)
                .textCase(.uppercase)
        }
        .gesture(
            DragGesture(minimumDistance: 0)
                .onChanged { _ in
                    isPressed = true
                }
                .onEnded { _ in
                    isPressed = false
                }
        )
        .sensoryFeedback(.impact, trigger: isPressed)
        .onChange(of: isPressed) {
            emu.input(role, pressed: isPressed)
        }
    }
}

private struct MenuInput: View {
    var body: some View {
        HStack {
            MenuButton(role: .select)
                .rotationEffect(.degrees(-30))
            MenuButton(role: .start)
                .rotationEffect(.degrees(-30))
        }
    }
}

private struct MenuButton: View {
    @Environment(GameBoy.self) private var emu

    let role: RugbyKit.Button

    @State private var isPressed = false

    var body: some View {
        VStack(spacing: 0) {
            Image("Menu")
                .overlay {
                    if isPressed {
                        Image("MenuShadow")
                    }
                }
            Text(String(describing: role))
                .font(.custom("Orbitron", size: 12))
                .fontWeight(.heavy)
                .foregroundStyle(.brand)
                .textCase(.uppercase)
        }
        .gesture(
            DragGesture(minimumDistance: 0)
                .onChanged { _ in
                    isPressed = true
                }
                .onEnded { _ in
                    isPressed = false
                }
        )
        .sensoryFeedback(.impact, trigger: isPressed)
        .onChange(of: isPressed) {
            emu.input(role, pressed: isPressed)
        }
    }
}
