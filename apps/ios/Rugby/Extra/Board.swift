//
//  Board.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-13.
//

import Foundation
import RugbyKit

extension RugbyKit.Board {
    /// Cartridge kind.
    var kind: String {
        switch self {
        case .none: "None"
        case .mbc1: "MBC1"
        case .mbc2: "MBC2"
        case .mbc3: "MBC3"
        case .mbc5: "MBC5"
        case .mbc6: "MBC6"
        case .mbc7: "MBC7"
        case .mmm01: "MMM01"
        case .m161: "M161"
        case .huC1: "HuC1"
        case .huC3: "HuC3"
        case .camera: "Camera"
        }
    }

    /// Supports battery.
    var power: Bool? {
        switch self {
        case .none(_, let power),
            .mbc1(_, let power),
            .mbc2(let power),
            .mbc3(_, let power, _),
            .mbc5(_, let power, _),
            .mmm01(_, let power):
            power
        default: nil
        }
    }

    /// Supports real-time clock.
    var clock: Bool? {
        switch self {
        case .mbc3(_, _, let clock):
            clock
        default: nil
        }
    }

    /// Supports rumble.
    var motor: Bool? {
        switch self {
        case .mbc5(_, _, let motor):
            motor
        default: nil
        }
    }
}
