import Foundation
import UIKit

class CartridgeCollectionViewCell: UICollectionViewCell {
    static let reuseIdentifier = "cartridgeCell"
    var image: UIImage
    var name: UILabel
    var cartridge: CartridgeViewModel?

    required init?(coder aDecoder: NSCoder) {
        self.image = UIImage()
        self.name = UILabel()
        super.init(coder: aDecoder)
    }
}
