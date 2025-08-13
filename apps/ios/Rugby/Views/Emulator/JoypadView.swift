//
//  JoypadView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-13.
//

import RugbyKit
import SwiftUI

struct JoypadView: View {
    typealias Callback = (RugbyKit.Button, Bool) -> Void

    /// Event callback.
    let call: Callback

    /// Region of the joypad to render.
    let part: Region

    init(_ call: @escaping Callback, part: Region = .full) {
        self.call = call
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
                        DPadInput(call: call)
                        Spacer()
                        GameInput(call: call)
                    }
                    MenuInput(call: call)
                }
            case .left:
                VStack {
                    Spacer()
                    DPadInput(call: call)
                    Spacer()
                    MenuInput(call: call)
                }
                .padding()
            case .right:
                VStack {
                    Spacer()
                    GameInput(call: call)
                    Spacer()
                    Spacer()
                }
                .padding()
            }
        }
    }
}

#Preview {
    JoypadView { (_, _) in }
}

private struct DPadInput: View {
    let call: JoypadView.Callback

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
            for role in next.subtracting(prev) {
                call(role, true)
            }
            for role in prev.subtracting(next) {
                call(role, false)
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
    let call: JoypadView.Callback

    var body: some View {
        HStack {
            GameButton(call: call, role: .b)
            GameButton(call: call, role: .a)
        }
        .rotationEffect(.degrees(-30))
    }
}

private struct GameButton: View {
    let call: JoypadView.Callback
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
                .foregroundStyle(.print)
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
            call(role, isPressed)
        }
    }
}

private struct MenuInput: View {
    let call: JoypadView.Callback

    var body: some View {
        HStack {
            MenuButton(call: call, role: .select)
                .rotationEffect(.degrees(-30))
            MenuButton(call: call, role: .start)
                .rotationEffect(.degrees(-30))
        }
    }
}

private struct MenuButton: View {
    let call: JoypadView.Callback
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
                .foregroundStyle(.print)
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
            call(role, isPressed)
        }
    }
}
