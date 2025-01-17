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
import zlib

struct GameInfo: View {
    @Environment(GameBoy.self) var emu
    @Environment(\.dismiss) var dismiss

    /// Game instance.
    @State var game: Game

    /// Game cartridge.
    var cart: Cartridge {
        (try? .init(rom: game.data))!
    }

    /// Cartridge header.
    var info: Header {
        cart.header()
    }

    var body: some View {
        List {
            Section {
                HStack {
                    Spacer()
                    GameIcon(game: game)
                        .shadow(radius: 4)
                    Spacer()
                }
                Text(game.name)
                    .font(.title2)
                    .fontWeight(.medium)
                Button("Play") {
                    dismiss()
                    // Only play if not playing anything
                    if emu.game == nil {
                        emu.play(game)
                    }
                }
                .bold()
                .buttonStyle(.borderedProminent)
                .clipShape(.rect(cornerRadius: .infinity))
            }
            .listRowSeparator(.hidden, edges: .top)
            Section("Information") {
                Row("Title") {
                    Text(info.title ?? "Unknown")
                }
                Row("Version") {
                    Text(info.version)
                }
                Row("Region") {
                    Text(info.region)
                }
                Row("Compatible") {
                    let support = [
                        (title: "DMG", allow: info.dmg, color: Color.blue),
                        (title: "CGB", allow: info.cgb, color: Color.purple),
                        (title: "SGB", allow: info.sgb, color: Color.red),
                    ]
                    .filter { $0.allow }
                    ForEach(support, id: \.title) {
                        Badge(title: $0.title, color: $0.color)
                    }
                }
            }
            Section("Cartridge") {
                Row("Kind") {
                    Text(info.cart)
                }
                Row("ROM") {
                    Text(info.romsz)
                }
                Row("RAM") {
                    Text(info.ramsz)
                }
            }
            Section("Checksum") {
                Row("Header") {
                    Text(String(format: "%02X", info.hchk))
                        .monospaced()
                        .textSelection(.enabled)
                }
                Row("Global") {
                    Text(String(format: "%04X", info.gchk))
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
        .environment(\.defaultMinListRowHeight, 32.5)
    }
}

#Preview {
    GameInfo(
        game: Game(
            path: Bundle.main.url(
                forResource: "roms/games/porklike/porklike",
                withExtension: "gb"
            )!
        )
    )
    .environment(GameBoy())
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
                    .foregroundStyle(Color.primary)
            }
        }
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
                .multilineTextAlignment(.trailing)
        }
        .font(.footnote)
        .textSelection(.enabled)
        .listRowSeparator(.hidden, edges: .top)
        .listRowSeparator(.visible, edges: .bottom)
        .alignmentGuide(.listRowSeparatorLeading) { row in
            row[.leading]
        }
        .alignmentGuide(.listRowSeparatorTrailing) { row in
            row[.trailing]
        }
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
