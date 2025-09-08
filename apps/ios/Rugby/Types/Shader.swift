//
//  Shader.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-09-09.
//

import Foundation
import SwiftUI

/// Video shader.
enum Shader: String, CaseIterable, CustomStringConvertible, Identifiable {
    /// Simulate an LCD.
    case lcd = "LCD"
    /// Scale2x algorithm.
    case scale2x = "Scale2x"
    /// Scale3x algorithm.
    case scale3x = "Scale3x"

    // impl CustomStringConvertible
    var description: String {
        rawValue
    }

    // impl Identifiable
    var id: Self { self }
}

extension Shader {
    /// Converts the `Shader` to its corresponding function.
    var scale: ShaderFunction {
        switch self {
        case .lcd:
            ShaderLibrary.lcd
        case .scale2x:
            ShaderLibrary.scale2x
        case .scale3x:
            ShaderLibrary.scale3x
        }
    }
}
