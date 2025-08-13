//
//  Logger.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-06.
//

import OSLog

/// Global logger.
let log = Logger()

#if DEBUG
    /// Define core logger.
    @_cdecl("log")
    func rugbyLog(level: UInt64, target: UnsafePointer<CChar>, message: UnsafePointer<CChar>) {
        // Format log message
        let msg = "[\(String(cString: target))]: \(String(cString: message))"
        // Write to console
        switch level {
        case 1:
            log.error("\(msg)")
        case 2:
            log.warning("\(msg)")
        case 3:
            log.info("\(msg)")
        case 4:
            log.debug("\(msg)")
        case 5:
            log.trace("\(msg)")
        default:
            log.notice("\(msg)")
        }
    }
#endif
