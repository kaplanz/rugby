//
//  GameItem.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-23.
//

import SwiftUI

struct GameItem: View {
    @Environment(GameBoy.self) private var emu
    @Environment(Library.self) private var lib

    /// Game instance.
    var game: Game

    /// About this game.
    @State private var detail = false
    /// Rename this game.
    @State private var rename = (show: false, text: String())
    /// Manage this game.
    @State private var manage = false
    /// Delete this game.
    @State private var delete = false

    var body: some View {
        VStack {
            GameIcon(game: game)
                .contextMenu {
                    Button("Play", systemImage: "play") {
                        emu.play(game)
                    }
                    Divider()
                    Button("Get Info", systemImage: "info.circle") {
                        detail.toggle()
                    }
                    RenameButton()
                    Button("Settings", systemImage: "gearshape") {
                        manage.toggle()
                    }
                    Divider()
                    Button("Delete", systemImage: "trash", role: .destructive) {
                        delete.toggle()
                    }
                }
            Text(game.name)
                .multilineTextAlignment(.center)
        }
        .onTapGesture {
            emu.play(game)
        }
        .renameAction {
            rename = (show: true, text: game.name)
        }
        .alert("Enter a new name for “\(game.name)”:", isPresented: $rename.show) {
            TextField(game.name, text: $rename.text)
            Button("Cancel", role: .cancel) {}
            Button("Rename") {
                lib.rename(game: game, to: rename.text)
            }
        }
        .alert(
            "Are you sure you want to delete “\(game.name)”?",
            isPresented: $delete
        ) {
            Button("Cancel", role: .cancel) {}
            Button("Delete", role: .destructive) {
                lib.delete(game: game)
            }
        } message: {
            Text("This item will be deleted immediately. You can't undo this action.")
        }
        .sheet(isPresented: $detail) {
            NavigationStack {
                GameInfo(game: game)
                    .navigationTitle("Info")
                    .navigationBarTitleDisplayMode(.inline)
                    .toolbar {
                        Button("Done") {
                            detail.toggle()
                        }
                        .bold()
                    }
            }
        }
        .sheet(isPresented: $manage) {
            NavigationStack {
                SettingsView()
                    .navigationTitle(game.name)
                    .navigationBarTitleDisplayMode(.inline)
                    .toolbar {
                        Button("Done") {
                            manage.toggle()
                        }
                        .bold()
                    }
            }
        }
    }
}

#Preview {
    GameItem(
        game: try! Game(
            path: Bundle.main.url(
                forResource: "roms/games/porklike/porklike",
                withExtension: "gb"
            )!
        )
    )
    .environment(GameBoy())
    .environment(Library())
}

struct GameIcon: View {
    @State var game: Game

    private var image: Image? {
        game.icon.flatMap(Image.init(uiImage:))
    }

    var body: some View {
        (image ?? Image(systemName: "questionmark.app"))
            .frame(width: 160, height: 144)
            .background(.ultraThinMaterial)
            .clipShape(.rect(cornerRadius: 15))
            .contentShape(.contextMenuPreview, .rect(cornerRadius: 15))
            .overlay {
                RoundedRectangle(cornerRadius: 15)
                    .stroke(.foreground.secondary)
            }
    }
}
