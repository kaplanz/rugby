//
//  Library.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-21.
//

import Foundation
import RugbyKit
import SwiftUI
import UniformTypeIdentifiers

extension UTType {
    public static let dmg = UTType(exportedAs: "dev.zakhary.rugby.gb")
    public static let cgb = UTType(exportedAs: "dev.zakhary.rugby.gbc")
    public static let sav = UTType(exportedAs: "dev.zakhary.rugby.sav")
}

@Observable
class Library {
    private let root = URL.documentsDirectory

    /// Library games.
    var games: [Game] = []

    /// Library errors.
    var error: [any Swift.Error] = []

    init() {
        reload()
    }

    /// Synchronizes the library from the filesystem.
    func reload() {
        // Filesystem operations
        let fs = FileManager.default
        do {
            // Ensure root directory exists
            try fs.createDirectory(
                at: root,
                withIntermediateDirectories: true
            )
            // Query game directories
            games = try fs.contentsOfDirectory(
                at: root,
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
                do {
                    return try Game(path: $0)
                } catch let error {
                    self.error.append(error)
                    return nil
                }
            }
        } catch let error {
            self.error.append(error)
        }
    }

    /// Inserts a new title into the library.
    func insert(src: URL) {
        // Acquire access permission
        if !src.startAccessingSecurityScopedResource() {
            fatalError("failed to securely access path: “\(src)”")
        }
        // Filesystem operations
        let fs = FileManager.default
        do {
            // Create game directory
            let dir = root.appending(path: src.deletingPathExtension().lastPathComponent)
            try fs.createDirectory(at: dir, withIntermediateDirectories: true)
            // Copy game ROM
            let rom = dir.appending(path: src.lastPathComponent)
            try fs.copyItem(at: src, to: rom)
        } catch let error {
            log.error("filesystem: \(error.localizedDescription)")
        }
        // Release access permission
        src.stopAccessingSecurityScopedResource()
        // Reload game library
        reload()
    }

    /// Renames a title in the library.
    func rename(game: Game, to name: String) {
        // Filesystem operations
        let fs = FileManager.default
        do {
            // Rename game files
            let src = game.path.deletingLastPathComponent()
            try fs.contentsOfDirectory(
                at: src,
                includingPropertiesForKeys: nil
            )
            .filter { $0.lastPathComponent != name }
            .forEach { file in
                let dest =
                    src
                    .appending(path: name)
                    .appendingPathExtension(file.pathExtension)
                try fs.moveItem(at: file, to: dest)
            }
            // Rename game directory
            let dst = src.deletingLastPathComponent().appendingPathComponent(name)
            try fs.moveItem(at: src, to: dst)
        } catch let error {
            log.error("filesystem: \(error.localizedDescription)")
        }
        // Reload game library
        reload()
    }

    /// Deletes a title from the library.
    func delete(game: Game) {
        // Filesystem operations
        let fs = FileManager.default
        do {
            // Remove game directory
            let dir = game.path.deletingLastPathComponent()
            try fs.removeItem(at: dir)
        } catch let error {
            log.error("filesystem: \(error.localizedDescription)")
        }
        // Reload game library
        reload()
    }
}
