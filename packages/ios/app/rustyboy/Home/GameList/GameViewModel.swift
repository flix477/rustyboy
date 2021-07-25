//
//  GameViewModel.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-20.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation

struct GameViewModel {
    let id: String
    let name: String
}

extension GameViewModel {
    static func from(game: Game) -> GameViewModel {
        return GameViewModel(id: game._id.stringValue, name: game.name)
    }
}
