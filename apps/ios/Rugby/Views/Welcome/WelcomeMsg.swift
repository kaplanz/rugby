//
//  WelcomeMsg.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-14.
//

import SwiftUI

struct WelcomeMsg: View {
    var body: some View {
        VStack {
            // Header
            AppIcon()
                .padding(4)
            Text("Welcome to \(Build.NAME)")
                .font(.title3)
                .fontWeight(.bold)
                .padding(.bottom, 24)
            // Body
            Item(
                title: "Your Library",
                about: """
                    Games and saves are accessible as plain files on your \
                    device, making it easy to import and export your library.
                    """
            ) {
                Image(systemName: "books.vertical.fill")
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .foregroundStyle(.cyan.gradient)
            }
            Item(
                title: "Play Your Way",
                about: """
                    Whether that's on touchscreen, keyboard, or your \
                    favourite game controller.
                    """
            ) {
                LazyVGrid(
                    columns: [
                        .init(spacing: 2),
                        .init(spacing: 2),
                    ],
                    spacing: 2,
                ) {
                    Group {
                        Image(systemName: "hand.tap.fill")
                            .resizable()
                        Image(systemName: "keyboard.fill")
                            .resizable()
                        Image(systemName: "arcade.stick.console.fill")
                            .resizable()
                        Image(systemName: "gamecontroller.fill")
                            .resizable()
                    }
                    .aspectRatio(contentMode: .fit)
                }
                .symbolRenderingMode(.hierarchical)
                .foregroundStyle(.green.gradient)
            }
            Item(
                title: "Cycle Accurate",
                about: """
                    Enjoy your games as you would on original hardware. \
                    Emulation accuracy will continue to improve until it's \
                    perfect.
                    """
            ) {
                Image(systemName: "target")
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .foregroundStyle(.pink.gradient)
            }
            Item(
                title: "Open Source",
                about: """
                    Open-source by design: explore the implementation, learn \
                    how it works, and contribute to the project.
                    """
            ) {
                Image(systemName: "chevron.left.slash.chevron.right")
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .foregroundStyle(.orange.gradient)
            }
            Spacer()
        }
        .fontDesign(.rounded)
        .padding(24)
    }
}

#Preview {
    WelcomeMsg()
}

private struct Item<Image: View>: View {
    let title: String
    let about: String
    @ViewBuilder
    let image: () -> Image

    var body: some View {
        HStack(alignment: .top) {
            image()
                .frame(width: 36, height: 36)
                .padding(4)
            VStack(alignment: .leading) {
                Text(title)
                    .font(.headline)
                Text(about)
                    .foregroundStyle(.secondary)
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
        .frame(maxWidth: .infinity)
    }
}
