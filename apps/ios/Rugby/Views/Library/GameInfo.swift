//
//  GameInfo.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-10.
//

import Algorithms
import CryptoKit
import RugbyKit
import SwiftUI
import UniformTypeIdentifiers
import zlib

struct GameInfo: View {
    @Environment(GameBoy.self) var emu
    @Environment(\.dismiss) var dismiss

    /// Game instance.
    @State var game: Game

    private var type: UTType? {
        UTType(filenameExtension: game.path.pathExtension)
    }

    var body: some View {
        List {
            // Header
            VStack(alignment: .leading) {
                // Icon
                HStack {
                    GameIcon(game: game)
                        .frame(maxHeight: 286)
                        .shadow(radius: 6, y: 4)
                }
                .frame(maxWidth: .infinity, alignment: .center)
                // Name
                Text(game.path.lastPathComponent)
                    .font(.title2)
                    .bold()
                // Type
                Text(
                    String(
                        format: "%@ - %d KiB",
                        type?.localizedDescription ?? "ROM image",
                        game.data.count / 1024,
                    )
                )
                .font(.body)
                .foregroundStyle(.secondary)
                // Play
                Button("Play") {
                    dismiss()
                    // Only play if not playing anything
                    if emu.game == nil {
                        emu.play(game)
                    }
                }
                .bold()
                .padding(.top, 8)
                .buttonStyle(.glassProminent)
            }
            .listRowSeparator(.hidden)
            // Information
            Section("Information") {
                Row("Title") {
                    Text(game.info.title ?? "Unknown")
                }
                Row("Version") {
                    Text(game.info.version)
                }
                Row("Region") {
                    Text(game.info.region)
                }
                Row("Compatibility") {
                    let support = [
                        (title: "DMG", allow: game.info.dmg, color: Color.blue),
                        (title: "CGB", allow: game.info.cgb, color: Color.purple),
                        (title: "SGB", allow: game.info.sgb, color: Color.red),
                    ]
                    .filter { $0.allow }
                    ForEach(support, id: \.title) {
                        Badge(title: $0.title, color: $0.color)
                    }
                }
            }
            // Cartridge
            Section("Cartridge") {
                Row("Kind") {
                    Text(game.info.cart)
                }
                Row("ROM") {
                    Text(game.info.romsz)
                }
                Row("RAM") {
                    Text(game.info.ramsz)
                }
            }
            // Checksum
            Section("Checksum") {
                Row("Header") {
                    Text(String(format: "%02X", game.info.hchk))
                        .monospaced()
                        .textSelection(.enabled)
                }
                Row("Global") {
                    Text(String(format: "%04X", game.info.gchk))
                        .monospaced()
                        .textSelection(.enabled)
                }
                Row("CRC32") {
                    let hash = game.data.withUnsafeBytes {
                        crc32(0, $0.bindMemory(to: UInt8.self).baseAddress, uInt(game.data.count))
                    }
                    let repr = String(format: "%08X", hash)
                    Text(repr)
                        .monospaced()
                        .textSelection(.enabled)
                }
                Row("MD5") {
                    let hash = Insecure.MD5.hash(data: game.data)
                    let repr = hash.map { String(format: "%02X", $0) }.chunks(ofCount: 4).map {
                        $0.joined()
                    }.joined(separator: " ")
                    Text(repr)
                        .monospaced()
                        .textSelection(.enabled)
                }
                Row("SHA-1") {
                    let hash = Insecure.SHA1.hash(data: game.data)
                    let repr = hash.map { String(format: "%02X", $0) }.chunks(ofCount: 4).map {
                        $0.joined()
                    }.joined(separator: " ")
                    Text(repr)
                        .monospaced()
                        .textSelection(.enabled)
                }
                Row("SHA-256") {
                    let hash = SHA256.hash(data: game.data)
                    let repr = hash.map { String(format: "%02X", $0) }.chunks(ofCount: 4).map {
                        $0.joined()
                    }.joined(separator: " ")
                    Text(repr)
                        .monospaced()
                        .textSelection(.enabled)
                }
            }
        }
        .listStyle(.plain)
        .environment(\.defaultMinListRowHeight, 0)
        .navigationTitle("Info")
        .navigationBarTitleDisplayMode(.inline)
    }
}

#Preview {
    if let game = Bundle
        .main
        .url(forResource: "porklike", withExtension: "gb")
        .flatMap({ try? Game(path: $0) })
    {
        GameInfo(game: game)
            .environment(GameBoy())
    }
}

private struct Section<Content: View>: View {
    let title: String?
    let content: Content

    init(@ViewBuilder content: () -> Content) {
        self.title = nil
        self.content = content()
    }

    init(_ title: String, @ViewBuilder content: () -> Content) {
        self.title = title
        self.content = content()
    }

    var body: some View {
        SwiftUI.Section {
            content
        } header: {
            if let title = title {
                Text(title)
                    .font(.title3)
                    .bold()
                    .foregroundStyle(Color.secondary)
            }
        }
        .listSectionSpacing(8)
    }
}

private struct Row<Content: View>: View {
    let title: String
    let value: Content

    init(_ title: String, @ViewBuilder content: () -> Content) {
        self.title = title
        self.value = content()
    }

    var body: some View {
        HStack {
            Text(title)
                .foregroundStyle(.secondary)
            Spacer()
            value
                .fixedSize(horizontal: false, vertical: true)
                .multilineTextAlignment(.trailing)
                .textSelection(.enabled)
        }
        .font(.footnote)
        .listRowSeparator(.visible)
        .alignmentGuide(.listRowSeparatorLeading) { row in
            row[.leading]
        }
        .alignmentGuide(.listRowSeparatorTrailing) { row in
            row[.trailing]
        }
        .listRowInsets(.vertical, 8)
    }
}

private struct Badge: View {
    let title: String
    let color: Color

    var body: some View {
        Text(title)
            .font(.caption2)
            .fontWeight(.medium)
            .foregroundStyle(color)
            .padding(3)
            .overlay {
                RoundedRectangle(cornerRadius: 4)
                    .stroke(color, lineWidth: 1.5)
            }
    }
}
