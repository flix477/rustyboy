import Foundation

enum GameLoadError: Error {
    case notAGameboyROM
    case errorOpeningFile
}

class HomeController {
    func onFileSelection(path: URL, _ completion: @escaping (Result<Gameboy, GameLoadError>) -> Void) {
        DispatchQueue.asyncResult(execute: { () -> Result<NSData, GameLoadError> in
            guard let data = NSData(contentsOf: path) else {
                return .failure(GameLoadError.errorOpeningFile)
            }

            return .success(data)
        }, completion: { result in
            switch result {
            case .success(let data):
                let buffer = [UInt8](data as Data)
                if let gameboy = Gameboy(buffer: buffer) {
                    completion(.success(gameboy))
                } else {
                    completion(.failure(GameLoadError.notAGameboyROM))
                }
            case .failure(let error):
                completion(.failure(error))
            }
        })
    }

    func getDocumentsPath() -> URL {
        return FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
    }

    func errorToString(_ error: GameLoadError) -> String {
        switch error {
        case .errorOpeningFile:
            return "An error occured while opening this file"
        case .notAGameboyROM:
            return "This file does not appear to be a valid GameBoy ROM"
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
