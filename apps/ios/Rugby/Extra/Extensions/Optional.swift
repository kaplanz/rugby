//
//  Optional.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-08.
//

import Foundation

extension Optional {
    /// Removes and returns the wrapped value, leaving `nil` in its place.
    mutating func take() -> Wrapped? {
        defer { self = nil }
        return self
    }
}
