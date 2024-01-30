import Foundation
import CoreGraphics

extension CGImage {
    static func from(savestateData: Data) -> CGImage? {
        let bitsPerComponent: Int = 8
        let bitsPerPixel: Int = 32
        let rgbaSize = 4
        let bitsPerRow:Int = Int(CGFloat(rgbaSize) * .screenWidth)

        let colorSpace = CGColorSpaceCreateDeviceRGB()
        let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedLast.rawValue)
        guard let provider = CGDataProvider(data: NSData(data: savestateData)) else { return nil }
        let renderingIntent = CGColorRenderingIntent.defaultIntent

        return CGImage(width: .screenWidth,
                       height: .screenHeight,
                       bitsPerComponent: bitsPerComponent,
                       bitsPerPixel: bitsPerPixel,
                       bytesPerRow: bitsPerRow,
                       space: colorSpace,
                       bitmapInfo: bitmapInfo,
                       provider: provider,
                       decode: nil,
                       shouldInterpolate: true,
                       intent: renderingIntent)
    }
}
