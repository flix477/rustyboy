//
//  FileManager_Extension.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-19.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation

extension FileManager {
    var documents: URL {
        return self.urls(for: .documentDirectory, in: .userDomainMask)[0]
    }

    var roms: URL {
        return documents.appendingPathComponent("ROMs")
    }

    func rom(withId id: String) -> URL {
        return roms.appendingPathComponent("\(id).gb")
    }

    var savestates: URL {
        return documents.appendingPathComponent("Savestates")
    }

    func savestates(forGameWithId id: String) -> URL {
        return savestates.appendingPathComponent(id)
    }

    func savestateFile(forGameWithId gameId: String, forSavestateWithId savestateId: String) -> URL {
        return savestates(forGameWithId: gameId).appendingPathComponent("\(savestateId).savestate")
    }

    func savestatePreview(forGameWithId gameId: String, forSavestateWithId savestateId: String) -> URL {
        return savestates(forGameWithId: gameId).appendingPathComponent("\(savestateId).jpg")
    }
}
