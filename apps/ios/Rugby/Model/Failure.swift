//
//  Failure.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-12.
//

import Foundation
import RugbyKit

/// Internal error.
@Observable
final class Failure {
    /// Current error.
    private(set) var this: Error? {
        willSet {
            if let prev = this {
                past.append(prev)
            }
        }
    }

    /// Error history.
    private(set) var past: [Error] = .init()

    /// Log a new error.
    func log(_ error: any Swift.Error) {
        this = .init(src: error)
    }

    /// Clear the main error.
    func clear() {
        this = nil
    }

    /// Clears all saved errors.
    func clearAll() {
        self.clear()
        past.removeAll()
    }
}

extension Failure {
    struct Error: Swift.Error {
        /// Error upstream source.
        let src: any Swift.Error
        /// Error logging timestamp.
        let clk: Date = .now

        /// Error source message.
        var msg: String {
            if let error = src as? RugbyKit.Error,
                let error = Mirror(reflecting: error).children.first?.value,
                let error = Mirror(reflecting: error).children.first?.value as? String
            {
                error
            } else {
                src.localizedDescription
            }
        }
    }
}
