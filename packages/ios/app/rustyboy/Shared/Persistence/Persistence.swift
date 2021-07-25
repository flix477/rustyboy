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
    func gameCount<D>() -> EnvIO<D, Error, Int>
    func games<D>() -> EnvIO<D, Error, [Game]>
    func add<D>(game: Game) -> EnvIO<D, Error, Void>
}

protocol SavestatesPersistence {
    func latestSavestate<D>(for game: Game) -> EnvIO<D, Error, Savestate?>
    func savestates<D>(for game: Game) -> EnvIO<D, Error, [Savestate]>
    func add<D>(savestate: Savestate, to game: Game) -> EnvIO<D, Error, Void>
    func savestate<D>(withId id: String) -> RIO<D, Savestate?>
}

protocol Persistence: GamesPersistence, SavestatesPersistence {}
