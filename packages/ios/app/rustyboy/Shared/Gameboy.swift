import Foundation
import CoreGraphics

class RawSavestate {
    let buffer: UnsafeMutableBufferPointer<UInt8>
    let preview: CGImage

    init(start: UnsafeMutablePointer<UInt8>?, count: UInt, screenBuffer: UnsafeMutablePointer<UInt8>) {
        self.buffer = UnsafeMutableBufferPointer(start: start, count: Int(count))
        self.preview = CGContext(data: screenBuffer,
                            width: Int(SCREEN_WIDTH),
                            height: Int(SCREEN_HEIGHT),
                            bitsPerComponent: 8,
                            bytesPerRow: 4 * Int(SCREEN_WIDTH),
                            space: CGColorSpace(name: CGColorSpace.sRGB)!,
                            bitmapInfo: CGBitmapInfo.byteOrder32Little.rawValue +
                                CGImageAlphaInfo.premultipliedFirst.rawValue)!.makeImage()!

    }

    deinit {
        vec_free(buffer.baseAddress, UInt(buffer.count))
    }
}

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

    func sendInput(buttonType: ButtonType, eventType: ButtonEventType) {
        gameboy_send_input(self.gameboyPointer, buttonType.toCore(), eventType.toCore())
    }

    func loadSavestate(buffer: [UInt8]) -> Bool {
        gameboy_load_savestate(gameboyPointer, buffer, UInt(buffer.count))
    }

    func dumpSavestate() -> RawSavestate {
        var pointer = UnsafeMutablePointer<UInt8>(nil)
        let size = gameboy_dump_savestate(gameboyPointer, &pointer)

        return RawSavestate(start: pointer, count: size, screenBuffer: bufferPointer!)
    }

    func reset() {
        gameboy_reset(gameboyPointer)
    }

    deinit {
        if let pointer = self.bufferPointer {
            buffer_free(pointer)
        }
        gameboy_free(self.gameboyPointer)
    }
}

enum ButtonEventType {
    case down
    case up

    func toCore() -> InputType {
        switch self {
        case .down:
            return InputDown
        case .up:
            return InputUp
        }
    }

    func toString() -> String {
        switch self {
        case .down:
            return "down"
        case .up:
            return "up"
        }
    }
}

enum ButtonType {
    case up
    case down
    case left
    case right
    case a
    case b
    case start
    case select

    func toCore() -> InputButton {
        switch self {
        case .down:
            return Down
        case .up:
            return Up
        case .left:
            return Left
        case .right:
            return Right
        case .a:
            return A
        case .b:
            return B
        case .start:
            return Start
        case .select:
            return Select
        }
    }

    func toString() -> String {
        switch self {
        case .down:
            return "down"
        case .up:
            return "up"
        case .left:
            return "left"
        case .right:
            return "right"
        case .a:
            return "a"
        case .b:
            return "b"
        case .start:
            return "start"
        case .select:
            return "select"
        }
    }
}
