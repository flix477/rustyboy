import Foundation

class Gameboy {
    var gameboyPointer: OpaquePointer

    init?(buffer: [UInt8]) {
        if let gameboy = create_gameboy(buffer, UInt(buffer.count)) {
            self.gameboyPointer = gameboy
        } else {
            return nil
        }
    }

    func runToVblank() -> [UInt8] {
        let screenBuffer = gameboy_run_to_vblank(self.gameboyPointer)

        let buffer = UnsafeMutableBufferPointer(start: screenBuffer, count: Int(BUFFER_SIZE))

        buffer_free(screenBuffer)

        // TODO: don't copy the data?
        return Array(buffer)
    }

    deinit {
        gameboy_free(self.gameboyPointer)
    }
}
