//
//  Palette.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import ColorTokensKit
import Foundation
import SwiftUI

/// Color palette.
enum Palette: String, CaseIterable, CustomStringConvertible, Identifiable {
    /// Nostalgic autumn sunsets.
    case autumnChill
    /// Aquatic blues.
    case blkAqu
    /// Winter snowstorm blues.
    case blueDream
    /// Combining cool and warm tones.
    case coldfire
    /// Soft and pastel coral hues.
    case coral
    /// Cold metallic darks with warm dated plastic lights.
    case demichrome
    /// Greens and warm browns with an earthy feel.
    case earth
    /// Creamsicle inspired orange.
    case iceCream
    /// Old-school dot-matrix display.
    case legacy
    /// Misty forest greens.
    case mist
    /// Simple blacks and whites.
    case mono
    /// William Morris's rural palette.
    case morris
    /// Waterfront at dawn.
    case purpleDawn
    /// Rusty red and brown hues.
    case rustic
    /// Deep and passionate purples.
    case velvetCherry

    /// Palette data.
    var data: Data {
        switch self {
        case .autumnChill:
            return .autumnChill
        case .blkAqu:
            return .blkAqu
        case .blueDream:
            return .blueDream
        case .coldfire:
            return .coldfire
        case .coral:
            return .coral
        case .demichrome:
            return .demichrome
        case .earth:
            return .earth
        case .iceCream:
            return .iceCream
        case .legacy:
            return .legacy
        case .mist:
            return .mist
        case .mono:
            return .mono
        case .morris:
            return .morris
        case .purpleDawn:
            return .purpleDawn
        case .rustic:
            return .rustic
        case .velvetCherry:
            return .velvetCherry
        }
    }

    /// Palette tint.
    var tint: Color {
        let base = Color.blue.toLCH()
        let tint = Color(rgb: data.c1).toLCH()
        return LCHColor(l: base.l, c: base.c, h: tint.h).toColor()
    }

    // impl CustomStringConvertible
    var description: String {
        rawValue.capitalized
    }

    // impl Identifiable
    var id: Self { self }
}

extension Palette {
    /// Palette data.
    final class Data: Identifiable, Sendable {
        let c0: UInt32
        let c1: UInt32
        let c2: UInt32
        let c3: UInt32

        init(c0: UInt32, c1: UInt32, c2: UInt32, c3: UInt32) {
            self.c0 = c0
            self.c1 = c1
            self.c2 = c2
            self.c3 = c3
        }
    }
}

extension Palette.Data: Collection, RandomAccessCollection {
    public var startIndex: Int { 0 }

    public var endIndex: Int { 4 }

    public func index(after i: Index) -> Index {
        i + 1
    }

    public subscript(position: Int) -> UInt32 {
        var value: UInt32
        switch position {
        case 0: value = c0
        case 1: value = c1
        case 2: value = c2
        case 3: value = c3
        default: fatalError()
        }
        return value
    }
}

extension Palette.Data {
    /// *Autumn Chill*.
    ///
    /// Nostalgic autumn sunsets.
    ///
    /// Based upon [Autumn Chill][source] by [Doph][author].
    ///
    /// [author]: https://lospec.com/dophsart
    /// [source]: https://lospec.com/palette-list/autumn-chill
    static let autumnChill: Palette.Data =
        .init(
            c0: 0xDAD3AF,
            c1: 0xD58863,
            c2: 0xC23A73,
            c3: 0x2C1E74
        )

    /// *Blk Aqu*.
    ///
    /// Aquatic blues.
    ///
    /// Based upon [Blk Aqu][source] by [BurakoIRL][author].
    ///
    /// [author]: https://lospec.com/blkirl
    /// [source]: https://lospec.com/palette-list/blk-aqu4
    static let blkAqu: Palette.Data =
        .init(
            c0: 0x9FF4E5,
            c1: 0x00B9BE,
            c2: 0x005F8C,
            c3: 0x002B59
        )

    /// *Blue Dream*.
    ///
    /// Winter snowstorm blues.
    ///
    /// Based upon [Blue Dream][source] by [Snowy Owl][author].
    ///
    /// [author]: https://lospec.com/snowy-owl
    /// [source]: https://lospec.com/palette-list/bluedream4
    static let blueDream: Palette.Data =
        .init(
            c0: 0xECF2CB,
            c1: 0x98D8B1,
            c2: 0x4B849A,
            c3: 0x1F285D
        )

    /// *Coldfire*.
    ///
    /// Combining cool and warm tones.
    ///
    /// Based upon [Coldfire][source] by [Kerrie Lake][author].
    ///
    /// [author]: https://lospec.com/kerrielake
    /// [source]: https://lospec.com/palette-list/coldfire-gb
    static let coldfire: Palette.Data =
        .init(
            c0: 0xF6C6A8,
            c1: 0xD17C7C,
            c2: 0x5B768D,
            c3: 0x46425E
        )

    /// *Coral*.
    ///
    /// Soft and pastel coral hues.
    ///
    /// Based upon [Coral][source] by [Yousurname][author].
    ///
    /// [author]: https://lospec.com/yousurname
    /// [source]: https://lospec.com/palette-list/coral-4
    static let coral: Palette.Data =
        .init(
            c0: 0xFFD0A4,
            c1: 0xF4949C,
            c2: 0x7C9AAC,
            c3: 0x68518A
        )

    /// *Demichrome*.
    ///
    /// Cold metallic darks with warm dated plastic lights.
    ///
    /// Based upon [2bit Demichrome][source] by [Space Sandwich][author].
    ///
    /// [author]: https://lospec.com/spacesandwich
    /// [source]: https://lospec.com/palette-list/2bit-demichrome
    static let demichrome: Palette.Data =
        .init(
            c0: 0xE9EFEC,
            c1: 0xA0A08B,
            c2: 0x555568,
            c3: 0x211E20
        )

    /// *Earth*.
    ///
    /// Greens and warm browns with an earthy feel.
    ///
    /// Based upon [Earth][source] by [Kerrie Lake][author].
    ///
    /// [author]: https://lospec.com/kerrielake
    /// [source]: https://lospec.com/palette-list/earth-gb
    static let earth: Palette.Data =
        .init(
            c0: 0xF5F29E,
            c1: 0xACB965,
            c2: 0xB87652,
            c3: 0x774346
        )

    /// *Ice Cream*.
    ///
    /// Creamsicle inspired orange.
    ///
    /// Based upon [Ice Cream][source] by [Kerrie Lake][author].
    ///
    /// [author]: https://lospec.com/kerrielake
    /// [source]: https://lospec.com/palette-list/ice-cream-gb
    static let iceCream: Palette.Data =
        .init(
            c0: 0xFFF6D3,
            c1: 0xF9A875,
            c2: 0xEB6B6F,
            c3: 0x7C3F58
        )

    /// *Legacy*.
    ///
    /// Old school dot-matrix display.
    ///
    /// Based upon [Legacy][source] by [Patrick Adams][author].
    ///
    /// [author]: https://www.deviantart.com/thewolfbunny64
    /// [source]: https://www.deviantart.com/thewolfbunny64/art/Game-Boy-Palette-DMG-Ver-808181265
    static let legacy: Palette.Data =
        .init(
            c0: 0x7F860F,
            c1: 0x577C44,
            c2: 0x365D48,
            c3: 0x2A453B
        )

    /// *Mist*.
    ///
    /// Misty forest greens.
    ///
    /// Based upon [Mist][source] by [Kerrie Lake][author].
    ///
    /// [author]: https://lospec.com/kerrielake
    /// [source]: https://lospec.com/palette-list/mist-gb
    static let mist: Palette.Data =
        .init(
            c0: 0xC4F0C2,
            c1: 0x5AB9A8,
            c2: 0x1E606E,
            c3: 0x2D1B00
        )

    /// *Mono*.
    ///
    /// Simple blacks and whites.
    static let mono: Palette.Data =
        .init(
            c0: 0xFFFFFF,
            c1: 0xAAAAAA,
            c2: 0x555555,
            c3: 0x000000
        )

    /// *Morris*.
    ///
    /// William Morris's rural palette.
    ///
    /// Based upon [Morris][source] by [Rabbit King][author].
    ///
    /// [author]: https://lospec.com/rabbitking
    /// [source]: https://lospec.com/palette-list/gb-morris
    static let morris: Palette.Data =
        .init(
            c0: 0xE5D8AC,
            c1: 0x7DB3AB,
            c2: 0x7C714A,
            c3: 0x264B38
        )

    /// *Purple Dawn*.
    ///
    /// Waterfront at dawn.
    ///
    /// Based upon [Purple Dawn][source] by [WildLeoKnight][author].
    ///
    /// [author]: https://lospec.com/wildleoknight
    /// [source]: https://lospec.com/palette-list/purpledawn
    static let purpleDawn: Palette.Data =
        .init(
            c0: 0xEEFDED,
            c1: 0x9A7BBC,
            c2: 0x2D757E,
            c3: 0x001B2E
        )

    /// *Rustic*.
    ///
    /// Rusty red and brown hues.
    ///
    /// Based upon [Rustic][source] by [Kerrie Lake][author].
    ///
    /// [author]: https://lospec.com/kerrielake
    /// [source]: https://lospec.com/palette-list/rustic-gb
    static let rustic: Palette.Data =
        .init(
            c0: 0xEDB4A1,
            c1: 0xA96868,
            c2: 0x764462,
            c3: 0x2C2137
        )

    /// *Velvet Cherry*.
    ///
    /// Deep and passionate purples.
    ///
    /// Based upon [Velvet Cherry][source] by [Klafooty][author].
    ///
    /// [author]: https://lospec.com/mallory
    /// [source]: https://lospec.com/palette-list/velvet-cherry-gb
    static let velvetCherry: Palette.Data =
        .init(
            c0: 0x9775A6,
            c1: 0x683A68,
            c2: 0x412752,
            c3: 0x2D162C
        )
}

extension Color {
    init(argb: UInt32) {
        let a = Double((argb >> 24) & 0xFF) / 255
        let r = Double((argb >> 16) & 0xFF) / 255
        let g = Double((argb >> 08) & 0xFF) / 255
        let b = Double((argb >> 00) & 0xFF) / 255
        self.init(red: r, green: g, blue: b, opacity: a)
    }

    init(rgb: UInt32) {
        self.init(argb: rgb | 0xFF00_0000)
    }
}
