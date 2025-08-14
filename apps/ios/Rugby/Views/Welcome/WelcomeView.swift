//
//  WelcomeView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-14.
//

import SwiftUI

struct WelcomeView: View {
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            VStack {
                // Header
                AppIcon()
                    .padding(4)
                Text("Welcome to \(Build.NAME)")
                    .font(.title3)
                    .fontWeight(.bold)
                    .padding(.bottom, 24)
                // Body
                WelcomeItem(
                    title: "Cycle Accurate",
                    about: """
                        Enjoy your games as you would on original hardware! \
                        The emulator core isn't done until it's perfect.
                        """
                ) {
                    Image(systemName: "target")
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .foregroundStyle(.red)
                }
                WelcomeItem(
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
                        Image(systemName: "hand.tap.fill")
                            .resizable()
                            .aspectRatio(contentMode: .fit)
                        Image(systemName: "keyboard.fill")
                            .resizable()
                            .aspectRatio(contentMode: .fit)
                        Image(systemName: "arcade.stick.console.fill")
                            .resizable()
                            .aspectRatio(contentMode: .fit)
                        Image(systemName: "gamecontroller.fill")
                            .resizable()
                            .aspectRatio(contentMode: .fit)
                    }
                    .symbolRenderingMode(.hierarchical)
                    .foregroundStyle(.blue)
                }
                WelcomeItem(
                    title: "Open Source",
                    about: """
                        Built in the open, for everyone. Explore the code, \
                        learn how it works, and help shape the future of the \
                        project.
                        """
                ) {
                    Image(systemName: "ellipsis.curlybraces")
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .foregroundStyle(.green)
                }
            }
            .fontDesign(.rounded)
            .padding(24)
            .toolbar {
                // Footer
                ToolbarItem(placement: .bottomBar) {
                    Button("Continue") { dismiss() }
                        .bold()
                        .buttonStyle(.glassProminent)
                        .padding(.vertical, 4)
                        .frame(maxWidth: .infinity)
                }
            }
        }
    }
}

#Preview {
    WelcomeView()
}

private struct WelcomeItem<Image: View>: View {
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
