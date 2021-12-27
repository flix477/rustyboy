//
//  MockPersistence.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-17.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import Bow
import BowEffects

struct MockPersistence {}

extension MockPersistence: GamesPersistence {
    func gameCount() -> Task<Int> {
        return Task.invoke(constant(0))
    }

    func games() -> Task<[Game]> {
        return Task.invoke(constant([]))
    }

    func add(game: Game) -> Task<Void> {
        return Task.invoke {}
    }
}

extension MockPersistence: SavestatesPersistence {
    func latestSavestate(for game: Game) -> Task<Savestate?> {
        return Task.invoke(constant(nil))
    }

    func savestates(for game: Game) -> Task<[Savestate]> {
        return Task.invoke(constant([]))
    }

    func add(savestate: Savestate, to game: Game) -> Task<Void> {
        return Task.invoke {}
    }

    func savestate(withId id: String) -> Task<Savestate?> {
        return Task.invoke(constant(nil))
    }
}

extension MockPersistence: Persistence {}
