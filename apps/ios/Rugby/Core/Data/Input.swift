//
//  Input.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-11.
//

import Foundation
import RugbyKit
import Synchronization

final class Input {
    /// Input events.
    let queue: Mutex<[(input: Button, state: Bool)]> = .init([])
}
