//
//  Failure.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-12.
//

import Foundation
import RugbyKit
import SwiftUI

/// Internal error.
@Observable
final class Failure {
    /// Current error.
    private(set) var this: [Error] = .init()

    /// Error history.
    private(set) var past: [Error] = .init()

    /// Log a new error.
    func log(_ error: any Swift.Error) {
        withAnimation {
            this.append(.init(src: error))
        }
    }

    /// Clear the main error.
    func clear() {
        withAnimation {
            past.append(contentsOf: this)
            this.removeAll()
        }
    }

    /// Clears all saved errors.
    func clearAll() {
        withAnimation {
            this.removeAll()
            past.removeAll()
        }
    }
}

extension Failure {
    struct Error: Swift.Error {
        /// Error global identifier.
        let eid: UUID = .init()
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

extension Failure.Error: Equatable {
    static func == (lhs: Failure.Error, rhs: Failure.Error) -> Bool {
        lhs.id == rhs.id
    }
}

extension Failure.Error: Identifiable {
    var id: UUID { eid }
}
