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
    var joypadInput: UInt8

    init?(buffer: [UInt8]) {
        self.joypadInput = 0
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
        self.bufferPointer = gameboy_run_to_vblank(self.gameboyPointer,
                                                   StepContext(pushed_keys: joypadInput,
                                                               serial_data_input: 0))!
        return self.bufferPointer!
    }

    func sendInput(buttonType: ButtonType, eventType: ButtonEventType) {
        let x = buttonType.toCore()
        if eventType == .down {
            joypadInput |= x
        } else {
            joypadInput &= ~x
        }
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

    func toCore() -> UInt8 {
        switch self {
        case .a:
            return 1
        case .b:
            return 2
        case .start:
            return 4
        case .select:
            return 8
        case .right:
            return 16
        case .left:
            return 32
        case .up:
            return 64
        case .down:
            return 128
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
