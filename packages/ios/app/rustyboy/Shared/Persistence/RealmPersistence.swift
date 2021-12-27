//
//  RealmPersistence.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-17.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import BowEffects
import Foundation
import RealmSwift

struct RealmPersistence {
    private var realm: Realm {
        // swiftlint:disable:next force_try
        return try! Realm()
    }

    private var _games: Results<Game> {
        return realm.objects(Game.self)
    }

    private var _savestates: Results<Savestate> {
        return realm.objects(Savestate.self)
    }

    private func _savestates(for game: Game) -> Results<Savestate> {
        return game.savestates.sorted(byKeyPath: "createdAt", ascending: false)
    }

    private func add(object: Object) -> Task<Void> {
        return write { realm in
            realm.add(object)
        }
    }

    private func write(_ perform: @escaping (Realm) throws -> Void) -> Task<Void> {
        return Task.invoke {
            try realm.write {
                try perform(realm)
            }
        }
    }
}

extension RealmPersistence: GamesPersistence {
    func gameCount() -> Task<Int> {
        return Task.invoke { _games.count }
    }

    func games() -> Task<[Game]> {
        return Task.invoke { Array(_games.sorted(byKeyPath: "name")) }
    }

    func add(game: Game) -> Task<Void> {
        return add(object: game)
    }
}

extension RealmPersistence: SavestatesPersistence {
    func latestSavestate(for game: Game) -> Task<Savestate?> {
        return Task.invoke { _savestates(for: game).first }
    }

    func savestates(for game: Game) -> Task<[Savestate]> {
        return Task.invoke { Array(_savestates(for: game)) }
    }

    func add(savestate: Savestate, to game: Game) -> Task<Void> {
        return write { _ in
            game.savestates.append(savestate)
        }
    }

    func savestate(withId id: String) -> Task<Savestate?> {
        return Task.invoke { realm.object(ofType: Savestate.self, forPrimaryKey: id) }
    }
}

extension RealmPersistence: Persistence {}
