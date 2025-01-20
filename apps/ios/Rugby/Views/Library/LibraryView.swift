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
        .refreshable {
            lib.reload()
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
                // Extract files on success
                guard case let .success(files) = result else {
                    return
                }
                // Iterate over selected files
                files
                    .filter { file in
                        // Acquire access permission
                        if !file.startAccessingSecurityScopedResource() {
                            fatalError("failed to securely access path: “\(file)”")
                        }
                        // Ensure valid ROM
                        do {
                            // Read the file data
                            let data = try Data(contentsOf: file)
                            // Try to construct a cartridge
                            let _ = try Cartridge(rom: data)
                            // Return on success
                            return true
                        } catch let error as RugbyKit.Error {
                            // Retain cartridge errors
                            lib.error.append(error)
                            return false
                        } catch let error {
                            // Crash on unknown errors
                            fatalError(error.localizedDescription)
                        }
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
