//
//  LibraryView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct LibraryView: View {
    @Environment(Library.self) private var lib

    /// Present file import dialog.
    @State private var file = false

    var body: some View {
        ScrollView {
            LazyVGrid(
                columns: [
                    GridItem(.adaptive(minimum: 172), alignment: .top)
                ]
            ) {
                ForEach(
                    lib.games.sorted(by: {
                        $0.name.lowercased() < $1.name.lowercased()
                    }), id: \.self
                ) { game in
                    GameItem(game: game)
                }
            }
        }
        .background(.regularMaterial)
        .toolbar {
            Button("Import", systemImage: "plus") {
                file.toggle()
            }
            .fileImporter(
                isPresented: $file,
                allowedContentTypes: [.dmg, .cgb],
                allowsMultipleSelection: true
            ) { result in
                switch result {
                case let .success(files):
                    for file in files {
                        lib.insert(src: file)
                    }
                case let .failure(error):
                    fatalError(error.localizedDescription)
                }
            }
        }
    }
}

#Preview {
    NavigationStack {
        LibraryView()
    }
    .environment(Library())
}
