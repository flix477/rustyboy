import Foundation
import UIKit

struct FontFamily {
    var semiBold: String
    var semiBoldItalic: String
}

class Theme {
    static let fontFamily = FontFamily(
        semiBold: "Cabin-SemiBold",
        semiBoldItalic: "Cabin-SemiBoldItalic"
    )

    static let tintColor = UIColor.init(red: 169, green: 60, blue: 111)
}
