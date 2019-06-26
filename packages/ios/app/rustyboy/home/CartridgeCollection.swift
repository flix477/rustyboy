//
//  CartridgeCollection.swift
//  rustyboy
//
//  Created by Felix Leveille on 2019-06-21.
//  Copyright © 2019 Félix Léveillé. All rights reserved.
//

import Foundation
import UIKit

class CartridgeCollection: NSObject, UICollectionViewDataSource, UICollectionViewDelegate {
    let cartridges: [CartridgeViewModel] = []

    func collectionView(_ collectionView: UICollectionView, numberOfItemsInSection section: Int) -> Int {
        return self.cartridges.count
    }

    func collectionView(_ collectionView: UICollectionView, cellForItemAt indexPath: IndexPath) -> UICollectionViewCell {
        let cell = collectionView.dequeueReusableCell(
            withReuseIdentifier: CartridgeCollectionViewCell.reuseIdentifier,
            for: indexPath
        ) as! CartridgeCollectionViewCell
        
        return cell
    }


}
