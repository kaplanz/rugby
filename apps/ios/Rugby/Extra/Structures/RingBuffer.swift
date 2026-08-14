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
        precondition(
            capacity > 0 && (capacity & (capacity - 1)) == 0,
            "capacity must be a positive power of two"
        )
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
    public var count: Int {
        let head = self.head.load(ordering: .acquiring)
        let tail = self.tail.load(ordering: .acquiring)
        return Int(min(tail &- head, size))
    }

    /// Checks if the queue is empty.
    public var isEmpty: Bool {
        count == 0
    }

    /// Clears the buffer.
    ///
    /// This must be called by the consumer.
    public func clear() {
        let tail = self.tail.load(ordering: .acquiring)
        let head = self.head.load(ordering: .relaxed)

        // Discard queued values, which may wrap
        let slot = Int(head & mask)
        let used = Int(tail &- head)
        let wrap = min(used, Int(size) - slot)
        data.advanced(by: slot).update(repeating: nil, count: wrap)
        data.update(repeating: nil, count: used - wrap)
        // Advance read pointer
        self.head.store(tail, ordering: .releasing)
    }

    /// Push a value. (Lock-free.)
    ///
    /// This must be called by the producer.
    ///
    /// # Returns
    ///
    /// Returns an indicator on whether the value was queued.
    ///
    /// # Note
    ///
    /// If buffer is full, the pushed value is dropped.
    @discardableResult
    public func push(_ value: T) -> Bool {
        let tail = self.tail.load(ordering: .relaxed)
        let head = self.head.load(ordering: .acquiring)

        // Ensure buffer is not full
        guard tail &- head < size else { return false }

        // Append value at tail
        data.advanced(by: Int(tail & mask)).pointee = .some(value)
        // Advance tail to publish
        self.tail.store(tail &+ 1, ordering: .releasing)

        return true
    }

    /// Pop a value. (Lock-free.)
    ///
    /// This must be called by the consumer.
    public func pop() -> T? {
        let head = self.head.load(ordering: .relaxed)
        let tail = self.tail.load(ordering: .acquiring)

        // Ensure buffer is non-empty
        guard tail &- head > 0 else { return nil }

        // Read value at head
        let value = data.advanced(by: Int(head & mask)).pointee.take()
        // Advance head to remove
        self.head.store(head &+ 1, ordering: .releasing)

        // Return obtained value
        return value
    }
}
