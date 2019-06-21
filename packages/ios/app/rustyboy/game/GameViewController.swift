import Foundation
import UIKit
import MetalKit

class GameViewController: UIViewController {
    var renderer: Renderer!
    var gameboy: Gameboy?
    lazy var mtkView: MTKView! = {
        let mtkView = MTKView()
        mtkView.translatesAutoresizingMaskIntoConstraints = false
        mtkView.device = MTLCreateSystemDefaultDevice()
        mtkView.colorPixelFormat = .bgra8Unorm

        return mtkView
    }()

    override func viewDidLoad() {
        super.viewDidLoad()
        self.view.addSubview(self.mtkView)
        self.renderer = Renderer(mtkView: self.mtkView)
        mtkView.delegate = self.renderer
    }
}
