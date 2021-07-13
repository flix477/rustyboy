import Foundation
import UIKit
import Bow
import BowEffects

enum GameLoadError: Error {
    case notAGameboyROM
    case errorOpeningFile
}

class HomeController {
//    func errorToString(_ error: GameLoadError) -> String {
//        switch error {
//        case .errorOpeningFile:
//            return "An error occured while opening this file"
//        case .notAGameboyROM:
//            return "This file does not appear to be a valid GameBoy ROM"
//        }
//    }

    func onFileSelection(path: URL) -> IO<GameLoadError, Gameboy> {
        return IO.invokeEither {
            guard let data = try? Data(contentsOf: path) else {
                return .left(.errorOpeningFile)
            }

            let buffer = [UInt8](data as Data)
            if let gameboy = Gameboy(buffer: buffer) {
                return .right(gameboy)
            } else {
                return .left(GameLoadError.notAGameboyROM)
            }
        }
    }
}

extension DispatchQueue {
    class func asyncResult<T>(execute: @escaping () -> T, completion: @escaping (T) -> Void) {
        DispatchQueue.global().async {
            let result = execute()
            DispatchQueue.main.async {
                completion(result)
            }
        }
    }
}
