//
//  Game.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-15.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import RealmSwift

class Game: Object {
    @Persisted(primaryKey: true) var _id: ObjectId
    @Persisted var name: String
    @Persisted var savestates: List<Savestate>

    convenience init(name: String) {
        self.init()
        self.name = name
     }
}

extension Game {
    var absolutePath: URL {
        return FileManager.default.rom(withId: _id.stringValue)
    }

    var savestatesAbsolutePath: URL {
        return FileManager.default.savestates(forGameWithId: _id.stringValue)
    }
}
