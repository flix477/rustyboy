// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "RustyboyCore",
    platforms: [.iOS(.v14)],
    products: [
        .library(
            name: "RustyboyCore",
            targets: ["RustyboyCore", "RustyboyCoreBindings"]),
    ],
    targets: [
        .target(name: "RustyboyCoreBindings", dependencies: ["RustyboyCore"]),
        .binaryTarget(name: "RustyboyCore", 
                      path: "../../../core/target/RustyboyCore.xcframework")
    ]
)
