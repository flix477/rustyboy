import Foundation
import UIKit
import SwiftUI

extension Color {
    static let tint = Color(red: 169, green: 60, blue: 111)
    static let buttonBackground = Color(red: 0, green: 0, blue: 0, opacity: 0.1)
    static let buttonPressedBackground = Color(red: 0, green: 0, blue: 0, opacity: 0.15)
}

extension Font {
    static let semiBold = { size in Font.custom("Cabin-Semibold", size: size) }
    static let semiBoldItalic = { size in Font.custom("Cabin-SemiboldItalic", size: size) }
}

extension Color {
    init(red: Int, green: Int, blue: Int) {
        self.init(red: Double(red) / 255, green: Double(green) / 255, blue: Double(blue) / 255)
    }

    init(red: Int, green: Int, blue: Int, opacity: Double) {
        self.init(red: Double(red) / 255, green: Double(green) / 255, blue: Double(blue) / 255, opacity: opacity)
    }
}

extension Gradient {
    static var primary: Gradient {
        return Gradient(colors: [Color(red: 36, green: 54, blue: 134),
                                 Color(red: 190, green: 109, blue: 147)])
    }
}
