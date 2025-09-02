//
//  LibraryView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct LibraryView: View {
    @Environment(Failure.self) private var err
    @Environment(Library.self) private var lib
    @Environment(\.scenePhase) private var scenePhase

    /// Present file importer.
    @State private var fileImport = false
    /// Searchable text query.
    @State private var query = String()

    /// Game library.
    private var games: [Game] {
        lib
            .games
            .filter {
                query.isEmpty || $0.name.localizedStandardContains(query)
            }
            .sorted(using: KeyPathComparator(\.name.localizedLowercase))
    }

    var body: some View {
        ScrollView {
            LazyVGrid(
                columns: [
                    GridItem(.adaptive(minimum: 125, maximum: 240), spacing: 12, alignment: .top)
                ],
                spacing: 12,
            ) {
                ForEach(games, id: \.self) { game in
                    LibraryItem(game: game) {
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
                            .background(.clear, in: .rect(cornerRadius: 12))
                    }
                }
            }
            .padding()
        }
        .navigationTitle("Library")
        .background(.background.secondary)
        .refreshable {
            do { try lib.reload() } catch { err.log(error) }
        }
        .searchable(text: $query)
        .searchToolbarBehavior(.minimize)
        .overlay {
            if !query.isEmpty && games.isEmpty {
                ContentUnavailableView.search(text: query)
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

#Preview {
    NavigationStack {
        LibraryView()
    }
    .environment(Failure())
    .environment(Library())
}
