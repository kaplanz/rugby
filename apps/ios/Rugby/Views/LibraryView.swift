//
//  LibraryView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import RugbyKit
import SwiftUI

struct LibraryView: View {
    @Environment(Library.self) private var lib

    /// Present file import dialog.
    @State private var file = false
    /// Searchable text query.
    @State private var query = String()

    /// Filtered and sorted games
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
                    GridItem(.adaptive(minimum: 125, maximum: 240), spacing: 16, alignment: .top)
                ],
            ) {
                ForEach(games, id: \.self) { game in
                    GameItem(game: game)
                }
            }
            .padding()
        }
        .navigationTitle("Library")
        .background(.background.secondary)
        .refreshable {
            lib.reload()
        }
        .searchable(text: $query)
        .overlay {
            if !query.isEmpty && games.isEmpty {
                ContentUnavailableView.search(text: query)
            }
        }
        .toolbar {
            Button("Import", systemImage: "plus") {
                file.toggle()
            }
            .fileImporter(
                isPresented: $file,
                allowedContentTypes: [.dmg, .cgb],
                allowsMultipleSelection: true
            ) { result in
                // Extract files on success
                guard case let .success(files) = result else {
                    return
                }
                // Iterate over selected files
                files
                    .filter { file in
                        // Ensure valid ROM
                        return (try? lib.precheck(url: file)) ?? false
                    }
                    .forEach { file in
                        // Attempt to add to library
                        lib.insert(src: file)
                        // Release access permission
                        file.stopAccessingSecurityScopedResource()
                    }
            }
        }
        .alert(
            "Error",
            isPresented: Binding(
                get: { lib.error.first != nil },
                set: { _ in }
            ), presenting: lib.error.first
        ) { _ in
            Button("OK", role: .cancel) {
                lib.error.removeFirst()
            }
        } message: { error in
            Text(String(describing: error))
        }
    }
}

#Preview {
    NavigationStack {
        LibraryView()
    }
    .environment(Library())
}
