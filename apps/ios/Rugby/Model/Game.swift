//
//  Game.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-09.
//

import Foundation
import SwiftUI
import UniformTypeIdentifiers

@Observable
final class Game {
    /// Game folder.
    let path: Folder

    init(path: URL) throws {
        self.path = .init(game: path)
        self.data = try Data(contentsOf: path)
    }

    /// Game folder structure.
    struct Folder {
        /// Folder root.
        var root: URL {
            game.deletingLastPathComponent()
        }

        /// Game ROM.
        var game: URL

        /// Game RAM.
        var save: URL {
            root.appending(component: name).appendingPathExtension("sav")
        }

        /// Title text.
        var name: String {
            root.lastPathComponent
        }

        /// Title type.
        var type: UTType? {
            .init(filenameExtension: game.pathExtension)
        }

        /// Title icon.
        var icon: URL {
            root.appending(component: name).appendingPathExtension("png")
        }
    }

    /// Favourite game.
    var star: Bool = false

    /// Cartridge title.
    var name: String {
        path.name
    }

    /// Cartridge ROM.
    let data: Data

    /// Cartridge RAM.
    var save: Data? {
        get {
            access(keyPath: \.save)
            return try? Data(contentsOf: path.save)
        }
        set {
            withMutation(keyPath: \.save) {
                try? newValue?.write(to: path.save)
            }
        }
    }

    /// Gameplay image.
    var icon: UIImage? {
        get {
            access(keyPath: \.icon)
            return (try? Data(contentsOf: path.icon)).flatMap { UIImage(data: $0) }
        }
        set {
            withMutation(keyPath: \.icon) {
                try? newValue?.pngData()?.write(to: path.icon)
            }
        }
    }
}

extension Game: Equatable {
    static func == (lhs: Game, rhs: Game) -> Bool {
        lhs.path.root == rhs.path.root
    }
}

extension Game: Hashable {
    func hash(into hasher: inout Hasher) {
        hasher.combine(path.root)
    }
}

extension Game: Identifiable {
    var id: URL { path.root }
}
