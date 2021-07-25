import Foundation
import UIKit
import Bow
import BowEffects

enum GameLoadError: Error {
    case notAGameboyROM
    case errorOpeningFile(Error)
}

struct HomeController {
    static func fetchGames<D: GamesPersistence>() -> EnvIO<D, Error, [Game]> {
        let persistence = EnvIO<D, Error, D>.var()
        let games = EnvIO<D, Error, [Game]>.var()

        return binding(
            persistence <- .ask(),
            games <- persistence.get.games(),
            yield: games.get)^
    }

    private static func loadData<D>(at url: URL) -> EnvIO<D, GameLoadError, Data> {
        return EnvIO.invoke { _ in try Data(contentsOf: url) }
            .mapError(GameLoadError.errorOpeningFile)
    }

    private static func copyFileToDocuments<D>(at url: URL, gameId: String) -> EnvIO<D, GameLoadError, URL> {
        return EnvIO.invoke { _ in
            try FileManager.default.createDirectory(at: FileManager.default.roms, withIntermediateDirectories: true)
            let newURL = FileManager.default.rom(withId: gameId)
            try FileManager.default.copyItem(at: url, to: newURL)

            return newURL
        }
        .mapError(GameLoadError.errorOpeningFile)
    }

    private static func makeGameboy<D>(data: Data) -> EnvIO<D, GameLoadError, Gameboy> {
        return EnvIO.invokeEither { _ in
            let buffer = [UInt8](data)
            if let gameboy = Gameboy(buffer: buffer) {
                return .right(gameboy)
            } else {
                return .left(GameLoadError.notAGameboyROM)
            }
        }
    }

    private static func loadLatestSavestate<D: HasPersistence>(game: Game, gameboy: Gameboy) -> RIO<D, Void> {
        return RIO.invoke { env in
            try Savestates.loadLatest().unsafeRunSync(with: EmulatorEnvironment(persistence: env.persistence,
                                                                                game: game,
                                                                                gameboy: gameboy))
        }
    }

    static func loadGame<D: HasPersistence>(game: Game) -> EnvIO<D, GameLoadError, Gameboy> {
        let persistence = EnvIO<D, GameLoadError, D>.var()
        let data = EnvIO<D, GameLoadError, Data>.var()
        let gameboy = EnvIO<D, GameLoadError, Gameboy>.var()

        return binding(
            persistence <- .ask(),
            data <- loadData(at: game.absolutePath),
            gameboy <- makeGameboy(data: data.get),
            |<-loadLatestSavestate(game: game, gameboy: gameboy.get).mapError(GameLoadError.errorOpeningFile),
            yield: gameboy.get)^
    }

    static func importGame<D: GamesPersistence>(at url: URL) -> EnvIO<D, GameLoadError, Game> {
        let persistence = EnvIO<D, GameLoadError, D>.var()
        let newURL = EnvIO<D, GameLoadError, URL>.var()
        let data = EnvIO<D, GameLoadError, Data>.var()
        let game = EnvIO<D, GameLoadError, Game>.var()

        func makeGame<D>(url: URL, data: Data) -> EnvIO<D, GameLoadError, Game> {
            return EnvIO.invokeEither { _ in
                let buffer = [UInt8](data)
                if Gameboy(buffer: buffer) != nil {
                    return .right(Game(name: url.lastPathComponent))
                } else {
                    return .left(GameLoadError.notAGameboyROM)
                }
            }
        }

        return binding(
            persistence <- .ask(),
            data <- loadData(at: url),
            game <- makeGame(url: url, data: data.get),
            newURL <- copyFileToDocuments(at: url, gameId: game.get._id.stringValue),
            |<-persistence.get.add(game: game.get)
                .mapError(GameLoadError.errorOpeningFile),
            yield: game.get)^
    }
}
