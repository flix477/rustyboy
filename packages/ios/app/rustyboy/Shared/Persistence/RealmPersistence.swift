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

    private func add<D>(object: Object) -> EnvIO<D, Error, Void> {
        return write { realm in
            realm.add(object)
        }
    }

    private func write<D>(_ perform: @escaping (Realm) throws -> Void) -> RIO<D, Void> {
        return EnvIO.invoke { _ in
            try realm.write {
                try perform(realm)
            }
        }
    }
}

extension RealmPersistence: GamesPersistence {
    func gameCount<D>() -> EnvIO<D, Error, Int> {
        return EnvIO.invoke { _ in _games.count }
    }

    func games<D>() -> EnvIO<D, Error, [Game]> {
        return EnvIO.invoke { _ in Array(_games.sorted(byKeyPath: "name")) }
    }

    func add<D>(game: Game) -> EnvIO<D, Error, Void> {
        return add(object: game)
    }
}

extension RealmPersistence: SavestatesPersistence {
    func latestSavestate<D>(for game: Game) -> EnvIO<D, Error, Savestate?> {
        return EnvIO.invoke { _ in _savestates(for: game).first }
    }

    func savestates<D>(for game: Game) -> EnvIO<D, Error, [Savestate]> {
        return EnvIO.invoke { _ in Array(_savestates(for: game)) }
    }

    func add<D>(savestate: Savestate, to game: Game) -> EnvIO<D, Error, Void> {
        return write { _ in
            game.savestates.append(savestate)
        }
    }

    func savestate<D>(withId id: String) -> RIO<D, Savestate?> {
        return RIO.invoke { _ in realm.object(ofType: Savestate.self, forPrimaryKey: id) }
    }
}

extension RealmPersistence: Persistence {}
