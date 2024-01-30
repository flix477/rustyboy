import Foundation
import MetalKit
import SwiftUI
import RustyboyCoreBindings

struct ScreenView: UIViewRepresentable {
    typealias UIViewType = MTKView
    let render: () -> Data
    private let device = MTLCreateSystemDefaultDevice()!

    func makeCoordinator() -> MTKViewDelegate {
        ScreenRenderer(device: device, onDraw: render)!
    }

    func makeUIView(context: Context) -> MTKView {
        let mtkView = MTKView()
        mtkView.device = device
        mtkView.colorPixelFormat = .bgra8Unorm
        mtkView.isOpaque = true
        mtkView.preferredFramesPerSecond = 60
        mtkView.delegate = context.coordinator
        mtkView.backgroundColor = .blue
        mtkView.autoResizeDrawable = true
        mtkView.drawableSize = mtkView.frame.size

        return mtkView
    }

    func updateUIView(_ uiView: MTKView, context: Context) {}
}

#Preview {
    ScreenView(render: {
        Data(repeating: 0xFF,
             count: .screenWidth * .screenHeight * 4)
    })
    .aspectRatio(.screenWidth / .screenHeight, contentMode: .fit)
    .background(Color.black)
}
