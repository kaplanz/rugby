//
//  Gamepad.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-10-16.
//

import Combine
import Foundation
import GameController
import SwiftUI

@Observable
final class Gamepad {
    // Current active controller.
    private(set) var main: GCController?
    // List connected controllers.
    private(set) var list: [GCController] = .init()

    private var changed = Set<AnyCancellable>()

    init() {
        // Connect
        NotificationCenter.default.publisher(for: .GCControllerDidConnect)
            .sink { [weak self] _ in
                withAnimation {
                    self?.list = GCController.controllers()
                }
            }
            .store(in: &changed)

        // Disconnect
        NotificationCenter.default.publisher(for: .GCControllerDidDisconnect)
            .sink { [weak self] _ in
                withAnimation {
                    self?.list = GCController.controllers()
                }
            }
            .store(in: &changed)

        // Activate
        NotificationCenter.default.publisher(for: .GCControllerDidBecomeCurrent)
            .sink { [weak self] notification in
                guard let controller = notification.object as? GCController else { return }
                withAnimation {
                    self?.main = controller
                }
            }
            .store(in: &changed)

        // Deactivate
        NotificationCenter.default
            .publisher(for: .GCControllerDidStopBeingCurrent)
            .sink { [weak self] notification in
                guard let controller = notification.object as? GCController else { return }
                assert(self?.main == controller)
                withAnimation {
                    self?.main = nil
                }
            }
            .store(in: &changed)
    }
}
