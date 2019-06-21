import Foundation
import MetalKit

class Renderer: NSObject, MTKViewDelegate {
    let device: MTLDevice
    let mtkView: MTKView
    let commandQueue: MTLCommandQueue
    let pipelineState: MTLRenderPipelineState
    let vertexBuffer: MTLBuffer
    let texture: MTLTexture
    var onDraw: (() -> UnsafeMutablePointer<UInt8>)?

    init?(mtkView: MTKView) {
        self.mtkView = mtkView
        self.device = mtkView.device!
        self.commandQueue = self.device.makeCommandQueue()!
        do {
            self.pipelineState = try Renderer.buildRenderPipelineWith(device: self.device, metalKitView: self.mtkView)
        } catch {
            print("Unable to compile render pipeline state: \(error)")
            return nil
        }

        let vertices = [
            Vertex(textureCoordinate: [0, 0], position: [-1, 1]),
            Vertex(textureCoordinate: [1, 0], position: [1, 1]),
            Vertex(textureCoordinate: [0, 1], position: [-1, -1]),
            Vertex(textureCoordinate: [1, 1], position: [1, -1])
        ]

        self.vertexBuffer = self.device.makeBuffer(
            bytes: vertices,
            length: vertices.count * MemoryLayout<Vertex>.stride,
            options: []
        )!

        let textureDescriptor = MTLTextureDescriptor()
        textureDescriptor.pixelFormat = self.mtkView.colorPixelFormat
        // todo: put those in rustyboy header
        textureDescriptor.width = 160
        textureDescriptor.height = 144
        self.texture = self.device.makeTexture(descriptor: textureDescriptor)!

        super.init()
    }

    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {

    }

    func draw(in view: MTKView) {
        if let buffer = self.onDraw?() {
            self.updateTextureWith(bufferPointer: buffer)
        }

        guard let commandBuffer = self.commandQueue.makeCommandBuffer() else { return }

        guard let renderPassDescriptor = view.currentRenderPassDescriptor else { return }
        renderPassDescriptor.colorAttachments[0].clearColor = MTLClearColorMake(0, 0, 0, 1)

        guard let renderEncoder = commandBuffer.makeRenderCommandEncoder(descriptor: renderPassDescriptor) else { return }
        renderEncoder.setRenderPipelineState(self.pipelineState)
        renderEncoder.setVertexBuffer(self.vertexBuffer, offset: 0, index: 0)
        renderEncoder.setFragmentTexture(self.texture, index: 0)
        renderEncoder.drawPrimitives(type: .triangleStrip, vertexStart: 0, vertexCount: 4)
        renderEncoder.endEncoding()

        commandBuffer.present(view.currentDrawable!)
        commandBuffer.commit()
    }

    class func buildRenderPipelineWith(device: MTLDevice, metalKitView: MTKView) throws -> MTLRenderPipelineState {
        let pipelineDescriptor = MTLRenderPipelineDescriptor()
        let library = device.makeDefaultLibrary()
        pipelineDescriptor.vertexFunction = library?.makeFunction(name: "vertexShader")
        pipelineDescriptor.fragmentFunction = library?.makeFunction(name: "fragmentShader")
        pipelineDescriptor.colorAttachments[0].pixelFormat = metalKitView.colorPixelFormat
        return try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
    }

    func updateTextureWith(bufferPointer: UnsafeMutablePointer<UInt8>) {
        let size = (160, 144)

        let region = MTLRegion(
            origin: MTLOrigin(x: 0, y: 0, z: 0),
            size: MTLSize(width: size.0, height: size.1, depth: 1)
        )

        let bytesPerRow = 4 * size.0

        self.texture.replace(
            region: region,
            mipmapLevel: 0,
            withBytes: bufferPointer,
            bytesPerRow: bytesPerRow
        )
    }
}
