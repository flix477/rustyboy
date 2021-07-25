//
//  Environment.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-19.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation

protocol HasGameboy {
    var gameboy: Gameboy { get }
    var game: Game { get }
}

protocol HasPersistence where PersistenceValue: Persistence {
    associatedtype PersistenceValue

    var persistence: PersistenceValue { get }
}
