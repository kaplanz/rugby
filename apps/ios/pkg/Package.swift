// swift-tools-version: 6.0
//
// The swift-tools-version declares the minimum version of Swift required to
// build this package.

import PackageDescription

let package = Package(
    name: "rugby",
    platforms: [
        .iOS(.v18),
        .macOS(.v15),
    ],
    products: [
        // Products define the executables and libraries a package produces,
        // making them visible to other packages.
        .library(
            name: "RugbyKit",
            targets: ["RugbyKit"]
        ),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module
        // or a test suite.
        //
        // Targets can depend on other targets in this package and products from
        // dependencies.
        .target(
            name: "RugbyKit",
            dependencies: ["rugbyFFI"]
        ),
        .binaryTarget(
            name: "rugbyFFI",
            // Swift packages importing this locally will not be able to import
            // the Rust core unless you use a relative path.
            path: "RugbyKit.xcframework"
        ),
    ]
)
