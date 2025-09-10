//
//  Shader.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-09.
//

import Foundation
import SwiftUI

/// Video shader.
enum Shader: String, CaseIterable {
    /// Simulate DMG's LCD.
    case dmg = "LCD (DMG)"
    /// Simulate CGB's LCD.
    case gbc = "LCD (GBC)"
    /// Simulate AGB's LCD.
    case gba = "LCD (GBA)"
    /// Scale2x algorithm.
    case scale2x = "Scale2x"
    /// Scale3x algorithm.
    case scale3x = "Scale3x"
}

extension Shader {
    /// Converts the `Shader` to its corresponding function.
    var shader: ShaderFunction {
        ShaderLibrary[dynamicMember: String(describing: self)]
    }
}

/// # Note
///
/// This is not implementing CustomStringConvertible, as that would override
/// the internals used by `Mirror`.
extension Shader {
    var description: String {
        rawValue
    }
}

extension Shader: Identifiable {
    var id: Self { self }
}
