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

    override var preferredStatusBarStyle: UIStatusBarStyle {
        return .lightContent
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        self.view.addSubview(self.mtkView)
        self.renderer = Renderer(mtkView: self.mtkView)
        mtkView.delegate = self.renderer

        self.mtkView.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        self.mtkView.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true
        self.mtkView.topAnchor.constraint(equalTo: self.view.topAnchor).isActive = true
        self.mtkView.bottomAnchor.constraint(equalTo: self.view.bottomAnchor).isActive = true
    }
}
