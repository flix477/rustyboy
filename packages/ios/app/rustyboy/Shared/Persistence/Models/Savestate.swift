//
//  Savestate.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-17.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import RealmSwift

class Savestate: Object {
    @Persisted(primaryKey: true) var _id: ObjectId
    @Persisted var createdAt: Date

    convenience init(createdAt: Date) {
        self.init()
        self.createdAt = createdAt
     }
}

extension Savestate {
    func absolutePath(game: Game) -> URL {
        return FileManager.default.savestateFile(forGameWithId: game._id.stringValue,
                                                 forSavestateWithId: _id.stringValue)
    }

    func absolutePreviewPath(game: Game) -> URL {
        return FileManager.default.savestatePreview(forGameWithId: game._id.stringValue,
                                                    forSavestateWithId: _id.stringValue)
    }
}
