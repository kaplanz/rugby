//
//  Palette.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import Foundation
import SwiftUI

/// Color palette.
enum Palette: String, CaseIterable, CustomStringConvertible, Identifiable {
    /// Simple blacks and whites.
    case mono
    /// Old-school dot-matrix display.
    case legacy

    /// Palette data.
    var data: Data {
        switch self {
        case .mono:
            return .mono
        case .legacy:
            return .legacy
        }
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
    class Data: Collection, Identifiable, RandomAccessCollection {
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

        /// Simple blacks and whites.
        static let mono: Data =
            .init(c0: 0xFFFFFF, c1: 0xAAAAAA, c2: 0x555555, c3: 0x000000)
        /// Old-school dot-matrix display.
        static let legacy: Data =
            .init(c0: 0x7F860F, c1: 0x577C44, c2: 0x365D48, c3: 0x2A453B)

        // impl Collection
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
            return value | 0xFF00_0000
        }
    }
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
