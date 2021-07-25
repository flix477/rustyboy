//
//  Savestates.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-20.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import Bow
import BowEffects

struct Savestates {
    static func loadLatest<D: HasPersistence & HasGameboy>() -> RIO<D, Void> {
        return RIO.accessM { env in
            env.persistence.latestSavestate(for: env.game)
        }.flatMap { savestate in
            if let savestate = savestate {
                return load(savestate: savestate)
            } else {
                return RIO.lazy()
            }
        }^
    }

    static func load<D: HasGameboy>(savestate: Savestate) -> RIO<D, Void> {
        return RIO.invoke { env in
            let data = try Data(contentsOf: savestate.absolutePath(game: env.game))
            env.gameboy.loadSavestate(buffer: [UInt8](data))
        }
    }

    static func dumpSavestate<D: HasPersistence & HasGameboy>() -> EnvIO<D, Error, Savestate> {
        let env = EnvIO<D, Error, D>.var()
        let now = EnvIO<D, Error, Date>.var()
        let savestate = EnvIO<D, Error, Savestate>.var()

        return binding(
            env <- .ask(),
            now <- Date.now().anyError.env(),
            savestate <- EnvIO.invoke { _ in
                let raw = env.get.gameboy.dumpSavestate()
                let savestate = Savestate(createdAt: now.get)

                let imageData = UIImage(cgImage: raw.preview).jpegData(compressionQuality: 0.8)!

                try FileManager.default.createDirectory(at: env.get.game.savestatesAbsolutePath,
                                                        withIntermediateDirectories: true)
                try imageData.write(to: savestate.absolutePreviewPath(game: env.get.game))
                try Data(raw.buffer).write(to: savestate.absolutePath(game: env.get.game))

                return savestate
            },
            |<-env.get.persistence.add(savestate: savestate.get, to: env.get.game),
            yield: savestate.get)^
    }

    static func savestates<D: HasPersistence>(for game: Game) -> RIO<D, [Savestate]> {
        let env = RIO<D, D>.var()
        let savestates = RIO<D, [Savestate]>.var()

        return binding(
            env <- .ask(),
            savestates <- env.get.persistence.savestates(for: game),
            yield: savestates.get)^
    }
}
