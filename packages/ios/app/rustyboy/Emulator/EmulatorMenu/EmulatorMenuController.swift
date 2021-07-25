//
//  EmulatorMenuController.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-20.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import Bow
import BowEffects

struct EmulatorMenuController {
    static func reset<D: HasGameboy>() -> RIO<D, Void> {
        return RIO.invoke { env in env.gameboy.reset() }
    }

    static func savestates<D: HasGameboy & HasPersistence>() -> RIO<D, [Savestate]> {
        let env = RIO<D, D>.var()
        let savestates = RIO<D, [Savestate]>.var()

        return binding(
            env <- .ask(),
            savestates <- env.get.persistence.savestates(for: env.get.game),
            yield: savestates.get)^
    }

    static func loadSavestate<D: HasGameboy>(savestate: Savestate) -> RIO<D, Void> {
        return Savestates.load(savestate: savestate)
    }

    static func dumpSavestate<D: HasGameboy & HasPersistence>() -> RIO<D, Savestate> {
        return Savestates.dumpSavestate()
    }
}
