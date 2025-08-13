//
//  Library.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-21.
//

import Foundation
import RugbyKit
import UniformTypeIdentifiers

extension UTType {
    public static let dmg = UTType(importedAs: "dev.zakhary.rugby.gb")
    public static let cgb = UTType(importedAs: "dev.zakhary.rugby.gbc")
    public static let sav = UTType(importedAs: "dev.zakhary.rugby.sav")
}

@Observable
final class Library {
    /// Filesystem root.
    static let root = URL.documentsDirectory

    /// Library games.
    var games: [Game] = []

    init() {
        try? reload()
    }

    /// Synchronizes the library from the filesystem.
    func reload() throws {
        // Filesystem operations
        let fs = FileManager.default
        // Ensure root directory exists
        try fs.createDirectory(
            at: Self.root,
            withIntermediateDirectories: true
        )
        // Query game directories
        games = try fs.contentsOfDirectory(
            at: Self.root,
            includingPropertiesForKeys: nil
        )
        // Find directories
        .filter {
            var isDir: ObjCBool = false
            fs.fileExists(atPath: $0.path(percentEncoded: false), isDirectory: &isDir)
            return isDir.boolValue
        }
        // Find game ROM
        .flatMap { dir in
            [UTType.dmg, UTType.cgb].compactMap({ ext in
                // Check if ROM exists with extension
                let rom = dir.appending(path: dir.lastPathComponent).appendingPathExtension(
                    for: ext)
                if fs.fileExists(atPath: rom.path(percentEncoded: false)) {
                    return rom
                } else {
                    return nil
                }
            })
        }
        // Construct game
        .compactMap {
            try Game(path: $0)
        }
    }

    /// Checks if a game ROM is valid.
    func check(url: URL) throws -> Bool {
        // Acquire access permission
        if !url.startAccessingSecurityScopedResource() {
            throw Self.Error.access
        }
        defer {
            url.stopAccessingSecurityScopedResource()
        }
        // Ensure valid ROM
        var valid = false
        // Read the file data
        let data = try Data(contentsOf: url)
        // Try to construct a cartridge
        let _ = try RugbyKit.check(data: data)
        // Mark as valid
        valid = true

        // Return validity
        return valid
    }

    /// Adds a new title to the library
    func add(url: URL) throws {
        // Acquire access permission
        if !url.startAccessingSecurityScopedResource() {
            fatalError("failed to securely access path: “\(url)”")
        }
        defer {
            url.stopAccessingSecurityScopedResource()
        }
        // Filesystem operations
        let fs = FileManager.default
        // Create game directory
        let dir = Self.root.appending(path: url.deletingPathExtension().lastPathComponent)
        try fs.createDirectory(at: dir, withIntermediateDirectories: true)
        // Copy game ROM
        let rom = dir.appending(path: url.lastPathComponent)
        try fs.copyItem(at: url, to: rom)

        // Reload game library
        try reload()
    }

    /// Copies a title in the library.
    func copy(game: Game, as name: String? = nil) throws {
        let fs = FileManager.default

        // Generate a unique name
        let name =
            name
            ?? {
                // Declare starting name, index
                var base = game.name  // base off of source's name
                var idx = 2  // start the copy index at 2
                // Update base, index on existing copies
                if let range = base.range(of: " ", options: .backwards),
                    let count = Int(base[range.upperBound...]),
                    count < 500
                {
                    base = String(base[..<range.lowerBound])
                    idx = count + 1
                }
                // Search for available name
                let root = Library.root
                var name: String
                repeat {
                    name = "\(base) \(idx)"
                    idx += 1
                } while fs.fileExists(
                    atPath: root.appending(component: name).path(percentEncoded: false)
                )
                // Return first available name
                return name
            }()

        // Copy game directory
        let src = game.path.root
        let dst = src.deletingLastPathComponent().appendingPathComponent(name)
        try fs.copyItem(at: src, to: dst)
        // Rename copied files
        try fs.contentsOfDirectory(at: dst, includingPropertiesForKeys: nil)
            .filter { $0.lastPathComponent != name }
            .forEach { old in
                let new = dst.appending(path: name).appendingPathExtension(old.pathExtension)
                try fs.moveItem(at: old, to: new)
            }

        // Reload game library
        try reload()
    }

    /// Renames a title in the library.
    func move(game: Game, to name: String) throws {
        // Filesystem operations
        let fs = FileManager.default
        // Rename game files
        let src = game.path.root
        try fs.contentsOfDirectory(at: src, includingPropertiesForKeys: nil)
            .filter { $0.lastPathComponent != name }
            .forEach { old in
                let new = src.appending(path: name).appendingPathExtension(old.pathExtension)
                try fs.moveItem(at: old, to: new)
            }
        // Rename game directory
        let dst = src.deletingLastPathComponent().appendingPathComponent(name)
        try fs.moveItem(at: src, to: dst)

        // Reload game library
        try reload()
    }

    /// Deletes a title from the library.
    func delete(game: Game) throws {
        // Filesystem operations
        let fs = FileManager.default
        // Remove game directory
        try fs.trashItem(at: game.path.root, resultingItemURL: nil)

        // Reload game library
        try reload()
    }
}

extension Library {
    /// An error caused by an library operations.
    enum Error: Swift.Error {
        /// Unable to access file.
        case access
    }
}
