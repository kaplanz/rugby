//
//  GameInfoView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-10.
//

import Algorithms
import CryptoKit
import RugbyKit
import SwiftUI
import zlib

struct GameInfoView: View {
    @Environment(Runtime.self) private var app
    @Environment(\.dismiss) private var dismiss

    /// Game instance.
    @State var game: Game

    /// Cartridge header.
    private var info: Header

    init(game: Game) {
        self.game = game
        self.info = try! RugbyKit.header(data: game.data)
    }

    var body: some View {
        List {
            // Header
            VStack(alignment: .leading) {
                // Icon
                HStack {
                    GameIcon(game: game)
                        .background(.clear, in: .rect(cornerRadius: 12))
                        .frame(maxHeight: 286)
                        .shadow(radius: 6, y: 4)
                }
                .frame(maxWidth: .infinity, alignment: .center)
                // Name
                Text(game.name)
                    .font(.title2)
                    .bold()
                // Type
                Text(
                    String(
                        format: "%@ - %@",
                        game.path.type?.localizedDescription ?? "ROM image",
                        game.data.count.formatted(.byteCount(style: .file))
                    )
                )
                .font(.body)
                .foregroundStyle(.secondary)
                // Play
                Button("Play") {
                    // Dismiss this view
                    dismiss()
                    // Start playing game
                    if app.game == nil {
                        app.play(game)
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
                    Text(info.about.title ?? "Unknown")
                }
                Row("Version") {
                    Text(
                        String(
                            format:
                                "v%d.%d",
                            ((info.about.version & 0xf0) >> 4) + 1,
                            info.about.version & 0x0f,
                        )
                    )
                }
                Row("Region") {
                    Text(String(describing: info.about.region).capitalized)
                }
                Row("Compatibility") {
                    let support = [
                        (title: "DMG", allow: info.compat.dmg, color: Color.blue),
                        (title: "CGB", allow: info.compat.cgb, color: Color.purple),
                        (title: "SGB", allow: info.compat.sgb, color: Color.red),
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
                    Text(info.board.kind)
                }
                Row("ROM") {
                    Text(info.memory.romsz.formatted(.byteCount(style: .memory)))
                }
                Row("RAM") {
                    Text(info.memory.ramsz.formatted(.byteCount(style: .memory)))
                }
                if let power = info.board.power {
                    Row("Battery") {
                        Text(power ? "Yes" : "No")
                    }
                }
                if let clock = info.board.clock {
                    Row("Clock") {
                        Text(clock ? "Yes" : "No")
                    }
                }
                if let motor = info.board.motor {
                    Row("Rumble") {
                        Text(motor ? "Yes" : "No")
                    }
                }
            }
            // Checksum
            Section("Checksum") {
                Row("Header") {
                    Text(String(format: "%02X", info.check.hchk))
                        .monospaced()
                        .textSelection(.enabled)
                }
                Row("Global") {
                    Text(String(format: "%04X", info.check.gchk))
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
        .scrollContentBackground(.visible)
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
        GameInfoView(game: game)
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
            .environment(Runtime())
    }
}
