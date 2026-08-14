//
//  Input.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-11.
//

import Foundation
import RugbyKit

extension Input {
    /// A single input event.
    typealias Event = (input: Button, state: Bool)
}

/// Input driver.
final class Input {
    /// Event storage.
    let queue: RingBuffer<Event> = .init(capacity: 64)
}
