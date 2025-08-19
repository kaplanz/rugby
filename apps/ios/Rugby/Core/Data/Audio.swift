//
//  Audio.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-06.
//

import AVFoundation
import Foundation
import RugbyKit

extension Audio {
    /// A single audio sample.
    typealias Sample = RugbyKit.Sample
}

/// Audio publisher.
@Observable
final class Audio {
    /// Sample storage.
    private let data: RingBuffer<Sample> = .init()
    /// Audio sampler.
    private let play: Playback

    init() {
        play = .init(data: data)
    }

    /// Publish an audio sample.
    func push(sample: Sample) {
        data.push(sample)
    }
}

@Observable
private final class Playback: @unchecked Sendable {
    private let engine: AVAudioEngine = .init()
    private var source: AVAudioSourceNode!
    private var worker: AVAudioConverter!
    private let sample: RingBuffer<Audio.Sample>
    private let buffer: AVAudioPCMBuffer!

    init(data: RingBuffer<Audio.Sample>) {
        // Retain sample buffer
        sample = data

        // Initialize converter
        let worker = AVAudioConverter(
            from: AVAudioFormat(
                commonFormat: .pcmFormatFloat32,
                sampleRate: Double(AUDIO),
                channels: 2,
                interleaved: false,
            )!,
            to: engine.outputNode.outputFormat(forBus: 0)
        )!
        self.worker = worker

        // Declare input buffer
        buffer = .init(pcmFormat: worker.inputFormat, frameCapacity: 1 << 12)

        // Create source node
        source = .init { [weak self] _, _, _, audioBufferList -> OSStatus in
            guard let self else { return noErr }

            // Resample data from emulator
            self.resample(ioData: audioBufferList)
            return noErr
        }

        // Connect audio graph
        engine.attach(source)
        engine.connect(source, to: engine.mainMixerNode, format: worker.outputFormat)

        // Start audio engine
        try? engine.start()
    }

    private func refill(numberOfFrames: AVAudioPacketCount) -> AVAudioPCMBuffer {
        // Get pointers to audio channels
        let data = (
            lt: buffer.floatChannelData![0],
            rt: buffer.floatChannelData![1],
        )

        // Sample input from emulator
        var counter = 0
        while counter < numberOfFrames {
            // Fetch sample from emulator
            guard let sample = sample.pop() else { break }

            // Add sample to input buffer
            data.lt[counter] = sample.lt
            data.rt[counter] = sample.rt

            // Increment sample counter
            counter += 1

            // Ensure no overflow
            guard counter < buffer.frameLength else { break }
        }

        // Update source buffer's frame length
        buffer.frameLength = .init(counter)

        // Return the refilled source buffer
        return buffer
    }

    private func resample(ioData: UnsafeMutablePointer<AudioBufferList>) {
        // Create output audio buffer
        guard
            let output = AVAudioPCMBuffer(
                pcmFormat: worker.outputFormat,
                bufferListNoCopy: ioData,
            )
        else { return }

        // Hold any conversion errors
        var error: NSError?

        // Conversion onto output buffer
        let status = worker.convert(to: output, error: &error) {
            [unowned self] numberOfFrames, inputStatus in
            // Only fill if we have enough data
            if sample.count > numberOfFrames {
                // Tell converter data is available
                inputStatus.pointee = .haveData
                // Refill buffer with samples
                return self.refill(numberOfFrames: numberOfFrames)
            } else {
                // Tell converter there's no data
                inputStatus.pointee = .noDataNow
                return nil
            }
        }

        // Log any conversion errors
        if status == .error, let error {
            log.error("failed to convert audio: \(error.localizedDescription)")
        }
    }
}
