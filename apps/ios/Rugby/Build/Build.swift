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
    static let NAME = Bundle.main.object(forInfoDictionaryKey: "CFBundleName") as! String
    /// Semantic version.
    static var VERSION: Version {
        var version = Version(
            Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as! String
        )!
        version.metadata.append(contentsOf: ["build", NUMBER])
        return version
    }
    /// Xcode build number.
    static let NUMBER = Bundle.main.object(forInfoDictionaryKey: "CFBundleVersion") as! String
    /// Compilation date.
    static var DATE: Date {
        let fmt = DateFormatter()
        fmt.dateFormat = "MMM dd yyyy HH:mm:ss"
        return fmt.date(from: "\(__OBJC_DATE__) \(__OBJC_TIME__)")!
    }
}
