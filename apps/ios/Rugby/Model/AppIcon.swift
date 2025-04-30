//
//  AppIcon.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-05-01.
//

import Foundation

enum AppIcon: String, CaseIterable, Hashable {
    case gamePak
    case gameBoy

    var appIcon: String? {
        switch self {
        case .gamePak:
            nil
        case .gameBoy:
            "GameBoy"
        }
    }

    var preview: String {
        appIcon ?? "GamePak"
    }

    static func from(appIcon: String?) -> Self? {
        return allCases.first { $0.appIcon == appIcon }
    }
}

extension AppIcon: CustomStringConvertible {
    var description: String {
        switch self {
        case .gameBoy:
            "Handheld"
        case .gamePak:
            "Cartridge"
        }
    }
}

extension AppIcon: Identifiable {
    var id: String {
        self.rawValue
    }
}
