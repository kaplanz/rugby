//
//  Build.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-14.
//

import Foundation
import SemVer

struct Build {
    /// Application name.
    static let NAME = Bundle.main.infoDictionary?["CFBundleName"] as! String
    /// Version number.
    static let VERSION = Version(
        Bundle.main.infoDictionary?["CFBundleShortVersionString"] as! String)!
    /// Compilation date.
    static var DATE: Date {
        let fmt = DateFormatter()
        fmt.dateFormat = "MMM dd yyyy HH:mm:ss"
        return fmt.date(from: "\(__OBJC_DATE__) \(__OBJC_TIME__)")!
    }
}
