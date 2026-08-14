//
//  LibraryView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import Algorithms
import SwiftUI

struct LibraryView: View {
    @Environment(Failure.self) private var err
    @Environment(Library.self) private var lib
    @Environment(\.scenePhase) private var scenePhase

    /// Present file importer.
    @State private var fileImport = false
    /// Searchable text query.
    @State private var query = String()
    /// Searching the library.
    @FocusState private var search: Bool

    /// Game library
    private var games: ([Game], [Game]) {
        lib
            .games
            .filter {
                query.isEmpty || $0.name.localizedStandardContains(query)
            }
            .sorted(using: KeyPathComparator(\.name.localizedLowercase))
            .partitioned(by: \.meta.star)
    }

    var body: some View {
        let (other, stars) = games
        List {
            if !stars.isEmpty {
                Section {
                    LazyVGrid(
                        columns: [
                            GridItem(.adaptive(minimum: 125, maximum: 240), spacing: 12)
                        ],
                        spacing: 12,
                    ) {
                        ForEach(stars, id: \.self) { game in
                            LibraryItem(game: game) {
                                Tile(game: game)
                            }
                        }
                    }
                    .listRowInsets(.all, 12)
                    .listRowBackground(Color.clear)
                    .listRowSeparator(.hidden)
                }
                .listSectionMargins(.vertical, 0)
            }
            Section("All Games") {
                ForEach(other, id: \.self) { game in
                    LibraryItem(game: game) {
                        Row(game: game)
                    }
                }
            }
        }
        .listStyle(.insetGrouped)
        .navigationTitle("Library")
        .searchable(text: $query)
        .searchFocused($search)
        .searchToolbarBehavior(.minimize)
        .overlay {
            if search && query.isEmpty {
                ContentUnavailableView.search
            } else if !query.isEmpty && other.isEmpty && stars.isEmpty {
                ContentUnavailableView.search(text: query)
            } else if lib.games.isEmpty {
                ContentUnavailableView {
                    Label("No Games", systemImage: "books.vertical")
                } description: {
                    Text("Import a ROM to add it to your library.")
                } actions: {
                    Button("Import", systemImage: "plus") {
                        fileImport.toggle()
                    }
                    .buttonStyle(.glassProminent)
                }
            }
        }
        .toolbar {
            ToolbarSpacer(.flexible, placement: .bottomBar)
            DefaultToolbarItem(kind: .search, placement: .bottomBar)
            ToolbarItem(placement: .bottomBar) {
                Button("Import", systemImage: "plus") {
                    fileImport.toggle()
                }
                .buttonStyle(.glassProminent)
                .fileImporter(
                    isPresented: $fileImport,
                    allowedContentTypes: [.dmg, .cgb],
                    allowsMultipleSelection: true
                ) { result in
                    // Extract files on success
                    guard case .success(let files) = result else {
                        return
                    }
                    // Iterate over selected files
                    files
                        .forEach { file in
                            do {
                                // Ensure valid ROM
                                try lib.check(url: file)
                                // Attempt to add to library
                                try lib.add(url: file)
                            } catch { err.log(error) }
                        }
                }
            }
        }
        .onChange(of: scenePhase) {
            if case .active = scenePhase {
                do { try lib.reload() } catch { err.log(error) }
            }
        }
    }
}

/// Favourite game tile.
private struct Tile: View {
    /// Game shown.
    let game: Game

    var body: some View {
        GameIcon(game: game)
            .overlay(alignment: .bottom) {
                Text(game.name)
                    .font(.footnote)
                    .bold()
                    .multilineTextAlignment(.center)
                    .lineLimit(2)
                    .minimumScaleFactor(0.5)
                    .padding(.horizontal, 12)
                    .padding(.vertical, 4)
                    .glassEffect(in: .rect(cornerRadius: 8))
                    .padding(4)
            }
            .containerShape(.rect(cornerRadius: 12))
    }
}

/// Library row.
private struct Row: View {
    /// Game shown.
    let game: Game

    var body: some View {
        HStack {
            GameIcon(game: game)
                .frame(width: 50, height: 45)
                .containerShape(.rect(cornerRadius: 6))
            Text(game.name)
                .lineLimit(2)
        }
    }
}

#Preview {
    NavigationStack {
        LibraryView()
    }
    .environment(Failure())
    .environment(Library())
}
