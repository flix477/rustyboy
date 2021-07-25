//
//  EmulatorController.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-19.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import Bow
import BowEffects

struct EmulatorController {
    static func dumpSavestate<D: HasPersistence & HasGameboy>() -> EnvIO<D, Error, Savestate> {
        return Savestates.dumpSavestate()
    }
}
