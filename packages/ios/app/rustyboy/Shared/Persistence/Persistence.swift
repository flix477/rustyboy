//
//  Persistence.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-17.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import BowEffects
import Foundation
import RealmSwift

protocol GamesPersistence {
    func gameCount() -> Task<Int>
    func games() -> Task<[Game]>
    func add(game: Game) -> Task<Void>
}

protocol SavestatesPersistence {
    func latestSavestate(for game: Game) -> Task<Savestate?>
    func savestates(for game: Game) -> Task<[Savestate]>
    func add(savestate: Savestate, to game: Game) -> Task<Void>
    func savestate(withId id: String) -> Task<Savestate?>
}

protocol Persistence: GamesPersistence, SavestatesPersistence {}
