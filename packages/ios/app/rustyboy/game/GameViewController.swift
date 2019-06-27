import Foundation
import UIKit
import MetalKit

class GameViewController: UIViewController {
    var renderer: Renderer!
    var gameboy: Gameboy!
    let controller = ControllerView()

    lazy var mtkView: MTKView! = {
        let mtkView = MTKView()
        mtkView.translatesAutoresizingMaskIntoConstraints = false
        mtkView.device = MTLCreateSystemDefaultDevice()
        mtkView.colorPixelFormat = .bgra8Unorm
        mtkView.heightAnchor.constraint(equalTo: mtkView.widthAnchor, multiplier: 9.0 / 10.0).isActive = true

        return mtkView
    }()

    override var preferredStatusBarStyle: UIStatusBarStyle {
        return .lightContent
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        self.view.addSubview(self.mtkView)
        self.view.addSubview(self.controller)
        self.renderer = Renderer(mtkView: self.mtkView)
        self.renderer.onDraw = {
            self.gameboy.runToVblank()
        }

        self.mtkView.delegate = self.renderer

        self.mtkView.topAnchor.constraint(equalTo: self.view.safeAreaLayoutGuide.topAnchor).isActive = true
        self.mtkView.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        self.mtkView.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true

        self.controller.topAnchor.constraint(equalTo: self.mtkView.bottomAnchor).isActive = true
        self.controller.bottomAnchor.constraint(equalTo: self.view.bottomAnchor).isActive = true
        self.controller.leadingAnchor.constraint(equalTo: self.view.leadingAnchor).isActive = true
        self.controller.trailingAnchor.constraint(equalTo: self.view.trailingAnchor).isActive = true

        self.controller.onButtonEvent = { button, eventType in
            self.gameboy.sendInput(buttonType: button, eventType: eventType)
        }
    }
}
