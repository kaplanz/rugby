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
class Game: Equatable, Hashable, Identifiable {
    let path: URL

    init(path: URL) {
        self.path = path
    }

    var name: String {
        path.deletingPathExtension().lastPathComponent
    }

    var data: Data {
        do {
            return try Data(contentsOf: path)
        } catch let error {
            fatalError(error.localizedDescription)
        }
    }

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
