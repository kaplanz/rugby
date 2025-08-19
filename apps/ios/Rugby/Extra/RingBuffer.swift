//
//  RingBuffer.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-15.
//

import Atomics
import Foundation

/// Lock-free ring buffer.
public final class RingBuffer<T> {
    /// Buffer capacity.
    private let size: UInt64
    /// Storage mask.
    private var mask: UInt64 {
        size - 1
    }

    /// Consumer pointer.
    ///
    /// Reads will occur at this index.
    private let head: ManagedAtomic<UInt64> = .init(0)
    /// Producer pointer.
    ///
    /// Writes will occur at this index.
    private let tail: ManagedAtomic<UInt64> = .init(0)

    /// Data storage buffer.
    private let data: UnsafeMutablePointer<T?>

    /// Create buffer with a power-of-two capacity.
    public init(capacity: Int = 1 << 12) {
        // Initialize capacity
        precondition((capacity & (capacity - 1)) == 0, "capacity must be power of two")
        size = UInt64(capacity)

        // Initialize storage
        data = UnsafeMutablePointer<T?>.allocate(capacity: capacity)
        data.initialize(repeating: nil, count: capacity)
    }

    deinit {
        // Deinitialize and deallocate storage
        data.deinitialize(count: Int(size))
        data.deallocate()
    }

    /// Number of items in the buffer.
    var count: Int {
        let tail = self.tail.load(ordering: .acquiring)
        let head = self.head.load(ordering: .acquiring)
        return Int(tail &- head)
    }

    /// Checks if the queue is empty.
    var isEmpty: Bool {
        let tail = self.tail.load(ordering: .acquiring)
        let head = self.head.load(ordering: .acquiring)
        return head == tail
    }

    /// Push a value. (Lock-free.)
    ///
    /// This should be called by the producer.
    ///
    /// # Note
    ///
    /// If buffer is full, the oldest value is dropped.
    public func push(_ value: T) {
        let tail = self.tail.load(ordering: .acquiring)
        var head = self.head.load(ordering: .acquiring)

        // Ensure buffer is not full
        if tail &- head >= size {
            // Drop oldest to make room
            head = self.head.loadThenWrappingIncrement(ordering: .acquiringAndReleasing)
            data.advanced(by: Int(head & mask)).pointee = .none
        }

        // Append value at tail
        data.advanced(by: Int(tail & mask)).pointee = .some(value)
        // Advance tail to publish
        self.tail.wrappingIncrement(ordering: .acquiringAndReleasing)
    }

    /// Pop a value. (Lock-free.)
    ///
    /// This should be called by the consumer.
    public func pop() -> T? {
        let tail = self.tail.load(ordering: .acquiring)
        let head = self.head.load(ordering: .acquiring)

        // Ensure buffer is non-empty
        guard tail > head else { return nil }

        // Read value at head
        let value = data.advanced(by: Int(head & mask)).pointee.take()
        // Advance head to remove
        self.head.wrappingIncrement(ordering: .acquiringAndReleasing)

        // Return obtained value
        return value
    }
}
