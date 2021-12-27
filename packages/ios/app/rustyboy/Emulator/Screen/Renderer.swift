import Foundation
import MetalKit

class Renderer: NSObject, MTKViewDelegate {
    private let device: MTLDevice
    private let pipelineState: MTLRenderPipelineState
    private let onDraw: (() -> UnsafeMutablePointer<UInt8>)?
    private let commandQueue: MTLCommandQueue
    private var fragmentShaderParams: FragmentShaderParams

    lazy var vertexBuffer: MTLBuffer = {
        let vertices = [
            Vertex(textureCoordinate: [0, 0], position: [-1, 1]),
            Vertex(textureCoordinate: [1, 0], position: [1, 1]),
            Vertex(textureCoordinate: [0, 1], position: [-1, -1]),
            Vertex(textureCoordinate: [1, 1], position: [1, -1])
        ]

        return self.device.makeBuffer(
            bytes: vertices,
            length: vertices.count * MemoryLayout<Vertex>.stride,
            options: []
        )!
    }()

    lazy var paramsBuffer: MTLBuffer = {
        return self.device.makeBuffer(
            bytes: &fragmentShaderParams,
            length: MemoryLayout<FragmentShaderParams>.size,
            options: []
        )!
    }()

    lazy var texture: MTLTexture = {
        let textureDescriptor = MTLTextureDescriptor()
        textureDescriptor.pixelFormat = .bgra8Unorm
        textureDescriptor.width = Int(SCREEN_WIDTH)
        textureDescriptor.height = Int(SCREEN_HEIGHT)
        return self.device.makeTexture(descriptor: textureDescriptor)!
    }()

    init?(device: MTLDevice, onDraw: (() -> UnsafeMutablePointer<UInt8>)? = nil) {
        self.device = device
        self.commandQueue = device.makeCommandQueue()!
        do {
            self.pipelineState = try Renderer.buildRenderPipelineWith(device: self.device)
        } catch {
            print("Unable to compile render pipeline state: \(error)")
            return nil
        }

        self.onDraw = onDraw
        let isDarkMode = UITraitCollection.current.userInterfaceStyle == .dark
        self.fragmentShaderParams = FragmentShaderParams(renderSize: [0, 0],
                                                         textureSize: [UInt32(SCREEN_WIDTH), UInt32(SCREEN_HEIGHT)],
                                                         darkMode: isDarkMode)

        super.init()
    }

    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        self.fragmentShaderParams.renderSize = [UInt32(size.width), UInt32(size.height)]
    }

    func draw(in view: MTKView) {
        if let buffer = self.onDraw?() {
            self.updateTextureWith(bufferPointer: buffer)
        }

        guard let commandBuffer = self.commandQueue.makeCommandBuffer(),
              let renderPassDescriptor = view.currentRenderPassDescriptor,
              let currentDrawable = view.currentDrawable else { return }

        renderPassDescriptor.colorAttachments[0].clearColor = MTLClearColorMake(0, 0, 0, 1)

        guard let renderEncoder = commandBuffer.makeRenderCommandEncoder(descriptor: renderPassDescriptor) else {
            return
        }

        renderEncoder.setRenderPipelineState(self.pipelineState)
        renderEncoder.setVertexBuffer(self.vertexBuffer, offset: 0, index: 0)
        renderEncoder.setFragmentTexture(self.texture, index: 0)
        renderEncoder.setFragmentBuffer(self.paramsBuffer, offset: 0, index: 0)
        renderEncoder.drawPrimitives(type: .triangleStrip, vertexStart: 0, vertexCount: 4)
        renderEncoder.endEncoding()

        commandBuffer.present(currentDrawable)
        commandBuffer.commit()
    }

    class func buildRenderPipelineWith(device: MTLDevice) throws -> MTLRenderPipelineState {
        let pipelineDescriptor = MTLRenderPipelineDescriptor()
        let library = device.makeDefaultLibrary()
        pipelineDescriptor.vertexFunction = library?.makeFunction(name: "vertexShader")
        pipelineDescriptor.fragmentFunction = library?.makeFunction(name: "fragmentShader")
        pipelineDescriptor.colorAttachments[0].pixelFormat = .bgra8Unorm
        return try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
    }

    func updateTextureWith(bufferPointer: UnsafeMutablePointer<UInt8>) {
        let region = MTLRegion(
            origin: MTLOrigin(x: 0, y: 0, z: 0),
            size: MTLSize(width: Int(SCREEN_WIDTH), height: Int(SCREEN_HEIGHT), depth: 1)
        )

        let bytesPerRow = 4 * Int(SCREEN_WIDTH)

        self.texture.replace(
            region: region,
            mipmapLevel: 0,
            withBytes: bufferPointer,
            bytesPerRow: bytesPerRow
        )
    }
}
