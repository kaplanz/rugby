//
//  LibraryItem.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-23.
//

import SwiftUI

struct LibraryItem<V: View>: View {
    @Environment(Runtime.self) private var app
    @Environment(Failure.self) private var err
    @Environment(Library.self) private var lib

    /// Game instance.
    var game: Game

    /// About this game.
    @State private var detail = false
    /// Rename this game.
    @State private var rename = (show: false, text: String())
    /// Delete this game.
    @State private var delete = false

    @ViewBuilder
    var view: () -> V

    var body: some View {
        view()
            .contextMenu {
                ControlGroup {
                    Button("Play", systemImage: "play.fill") {
                        do { try app.play(game) } catch { err.log(error) }
                    }
                    ShareLink(item: game.path.game) {
                        Label("Share", systemImage: "square.and.arrow.up.fill")
                    }
                }
                Section {
                    Button("Get Info", systemImage: "info.circle") {
                        detail.toggle()
                    }
                    RenameButton()
                    Button("Duplicate", systemImage: "plus.square.on.square") {
                        withAnimation {
                            do { try lib.copy(game: game) } catch { err.log(error) }
                        }
                    }
                }
                Section {
                    Button("Show in Enclosing Folder", systemImage: "arrow.up.folder") {
                        if let url = URL(
                            string: game.path.root.absoluteString
                                .replacingOccurrences(of: "file://", with: "shareddocuments://"))
                        {
                            UIApplication.shared.open(url)
                        }
                    }
                }
                Section {
                    !game.star
                        ? Button("Favourite", systemImage: "star") {
                            withAnimation { game.star.toggle() }
                        }
                        : Button("Unfavourite", systemImage: "star.slash") {
                            withAnimation { game.star.toggle() }
                        }
                }
                .disabled(true)
                Section {
                    Button("Delete", systemImage: "trash", role: .destructive) {
                        delete.toggle()
                    }
                }
            }
            .onTapGesture {
                do { try app.play(game) } catch { err.log(error) }
            }
            .renameAction {
                rename = (show: true, text: game.name)
            }
            .alert("Enter a new name for “\(game.name)”:", isPresented: $rename.show) {
                TextField(game.name, text: $rename.text)
                Button("Cancel", role: .cancel) {}
                Button("Rename") {
                    withAnimation {
                        do { try lib.move(game: game, to: rename.text) } catch { err.log(error) }
                    }
                }
            }
            .alert("Are you sure you want to delete “\(game.name)”?", isPresented: $delete) {
                Button("Cancel", role: .cancel) {}
                Button("Delete", role: .destructive) {
                    withAnimation {
                        do { try lib.delete(game: game) } catch { err.log(error) }
                    }
                }
            } message: {
                Text("This item will be moved to the trash.")
            }
            .sheet(isPresented: $detail) {
                NavigationStack {
                    GameInfoView(game: game)
                        .toolbar {
                            Button("Done", systemImage: "checkmark", role: .confirm) {
                                detail.toggle()
                            }
                            .bold()
                        }
                }
            }
    }
}

#Preview {
    if let game = Bundle
        .main
        .url(forResource: "porklike", withExtension: "gb")
        .flatMap({ try? Game(path: $0) })
    {
        LibraryItem(game: game) {
            GameIcon(game: game)
                .frame(width: 160, height: 144)
        }
        .environment(Runtime())
        .environment(Failure())
        .environment(Library())
    }
}
