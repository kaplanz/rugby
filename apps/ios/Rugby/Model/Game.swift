//
//  Game.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-09.
//

import RugbyKit
import SwiftUI
import UniformTypeIdentifiers

@Observable
class Game: Equatable, Hashable, Identifiable {
    /// Game ROM path.
    let path: URL

    /// Game ROM data.
    let data: Data

    /// Game cartridge.
    let cart: Cartridge

    init(path: URL) throws {
        // Initialize game path
        self.path = path
        // Initialize game data
        self.data = try Data(contentsOf: path)
        // Initialize game cartridge
        self.cart = try Cartridge(rom: data)
    }

    /// Game name.
    var name: String {
        path.deletingPathExtension().lastPathComponent
    }

    /// Game information.
    var info: Header {
        cart.header()
    }

    /// Game busy status.
    var busy: Bool {
        cart.busy()
    }

    /// Game icon.
    var icon: UIImage? {
        get {
            access(keyPath: \.icon)
            let path = path.deletingPathExtension().appendingPathExtension("png")
            let data = try? Data(contentsOf: path)
            return data.flatMap { UIImage(data: $0) }
        }
        set {
            withMutation(keyPath: \.icon) {
                let path = path.deletingPathExtension().appendingPathExtension("png")
                do {
                    try newValue?.pngData()?.write(to: path)
                } catch let error {
                    fatalError(error.localizedDescription)
                }
            }
        }
    }

    // impl Equatable
    static func == (lhs: Game, rhs: Game) -> Bool {
        lhs.path == rhs.path
    }

    // impl Hashable
    func hash(into hasher: inout Hasher) {
        hasher.combine(path)
    }

    // impl Identifiable
    var id: URL { path }
}
