//
//  Atomic.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-07.
//

import Atomics
import Foundation

/// Atomic field member.
@propertyWrapper
struct Atomic<T: AtomicValue> where T.AtomicRepresentation.Value == T {
    private let atomic: ManagedAtomic<T>

    init(wrappedValue value: T) {
        atomic = .init(value)
    }

    var wrappedValue: T {
        get { atomic.load(ordering: .acquiring) }
        set { atomic.store(newValue, ordering: .releasing) }
    }
}
