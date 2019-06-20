import Foundation

enum GameLoadError: Error {
    case notAGameboyROM
    case errorOpeningFile
}

class HomeController {
    func onFileSelection(path: URL) -> Result<Gameboy, GameLoadError> {
        guard let data = NSData(contentsOf: path) else {
            return .failure(GameLoadError.errorOpeningFile)
        }

        let buffer = [UInt8](data as Data)

        if let gameboy = Gameboy(buffer: buffer) {
            return .success(gameboy)
        } else {
            return .failure(GameLoadError.notAGameboyROM)
        }
    }
}
