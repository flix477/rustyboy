import Foundation

class Gameboy {
    var gameboyPointer: OpaquePointer
    var bufferPointer: UnsafeMutablePointer<UInt8>?

    init?(buffer: [UInt8]) {
        if let gameboy = create_gameboy(buffer, UInt(buffer.count)) {
            self.gameboyPointer = gameboy
        } else {
            return nil
        }
    }

    func runToVblank() -> UnsafeMutablePointer<UInt8> {
        if let pointer = self.bufferPointer {
            buffer_free(pointer)
        }
        self.bufferPointer = gameboy_run_to_vblank(self.gameboyPointer)!
        return self.bufferPointer!
    }

    deinit {
        gameboy_free(self.gameboyPointer)
    }
}
