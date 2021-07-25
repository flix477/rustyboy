//
//  SavestateViewModel.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-20.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import Bow
import BowEffects
import UIKit

struct SavestateViewModel {
    let id: String
    let createdAt: Date
    let image: UIImage
}

extension SavestateViewModel {
    var formattedDate: String {
        return String(describing: createdAt)
    }

    static func from(savestate: Savestate) -> EnvIO<Game, Error, SavestateViewModel> {
        return EnvIO.invoke { game in
            return SavestateViewModel(id: savestate._id.stringValue,
                                      createdAt: savestate.createdAt,
                                      image: UIImage(contentsOfFile: savestate.absolutePreviewPath(game: game).path)!)
        }
    }
}
