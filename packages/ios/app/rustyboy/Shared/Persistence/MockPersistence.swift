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
    func gameCount<D>() -> EnvIO<D, Error, Int> {
        return EnvIO.invoke(constant(0))
    }

    func games<D>() -> EnvIO<D, Error, [Game]> {
        return EnvIO.invoke(constant([]))
    }

    func add<D>(game: Game) -> EnvIO<D, Error, Void> {
        return EnvIO.invoke { _ in }
    }
}

extension MockPersistence: SavestatesPersistence {
    func latestSavestate<D>(for game: Game) -> EnvIO<D, Error, Savestate?> {
        return EnvIO.invoke(constant(nil))
    }

    func savestates<D>(for game: Game) -> EnvIO<D, Error, [Savestate]> {
        return EnvIO.invoke(constant([]))
    }

    func add<D>(savestate: Savestate, to game: Game) -> EnvIO<D, Error, Void> {
        return EnvIO.invoke { _ in }
    }

    func savestate<D>(withId id: String) -> RIO<D, Savestate?> {
        return EnvIO.invoke(constant(nil))
    }
}

extension MockPersistence: Persistence {}
