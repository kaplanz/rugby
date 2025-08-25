//
//  Media.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-08.
//

import Foundation
import RugbyKit
import Synchronization

/// Media driver.
final class Media: Sendable {
    /// Request events.
    internal let request: Mutex<Request?> = .init(nil)
    internal enum Request {
        case insert(Cartridge)
        case eject
    }

    /// Response event.
    internal let respond: Mutex<Response?> = .init(nil)
    internal enum Response {
        case eject(Cartridge)
        case empty
    }

    /// Insert a cartridge.
    func insert(_ cart: Cartridge) {
        // Insert cartridge
        request.withLock { event in
            event = .insert(cart)
        }
    }

    /// Eject the cartridge.
    func eject() -> Cartridge? {
        // Request ejection
        request.withLock { event in
            event = .eject
        }
        // Return cartridge
        while true {
            // Loop until response
            guard let event = respond.withLock({ $0 }) else {
                continue
            }
            // Handle response
            switch event {
            case .eject(let cart):
                return cart
            case .empty:
                return nil
            }
        }
    }
}
