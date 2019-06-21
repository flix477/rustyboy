import Foundation
import MetalKit

class Renderer: NSObject, MTKViewDelegate {
    let device: MTLDevice
    let mtkView: MTKView
    let commandQueue: MTLCommandQueue
    let pipelineState: MTLRenderPipelineState
    let vertexBuffer: MTLBuffer

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
            Vertex(color: [1, 0, 0, 1], pos: [-1, -1]),
            Vertex(color: [0, 1, 0, 1], pos: [0, 1]),
            Vertex(color: [0, 0, 0, 1], pos: [1, -1])
        ]

        self.vertexBuffer = self.device.makeBuffer(
            bytes: vertices,
            length: vertices.count * MemoryLayout<Vertex>.stride,
            options: []
        )!

        super.init()
    }

    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {

    }

    func draw(in view: MTKView) {
        guard let commandBuffer = self.commandQueue.makeCommandBuffer() else { return }

        guard let renderPassDescriptor = view.currentRenderPassDescriptor else { return }
        renderPassDescriptor.colorAttachments[0].clearColor = MTLClearColorMake(1, 0, 0, 1)

        guard let renderEncoder = commandBuffer.makeRenderCommandEncoder(descriptor: renderPassDescriptor) else { return }
        renderEncoder.setRenderPipelineState(self.pipelineState)
        renderEncoder.setVertexBuffer(self.vertexBuffer, offset: 0, index: 0)
        renderEncoder.drawPrimitives(type: .triangle, vertexStart: 0, vertexCount: 3)
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
}
