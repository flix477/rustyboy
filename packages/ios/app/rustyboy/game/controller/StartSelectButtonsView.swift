import Foundation
import UIKit

class StartSelectButtonsView: UIView {
    static let buttonSize = (CGFloat(84), CGFloat(72))
    static let buttonImageSize = (CGFloat(76), CGFloat(56))

    override open class var requiresConstraintBasedLayout: Bool {
        return true
    }

    var onButtonEvent: ((ButtonType, ButtonEventType) -> ())?

    lazy var startButton: UIButton = {
        let button = StartSelectButtonsView.createButton()
        button.addTarget(self, action: #selector(self.startButtonDown), for: .touchDown)
        button.addTarget(self, action: #selector(self.startButtonUp), for: .touchUpInside)
        button.addTarget(self, action: #selector(self.startButtonDown), for: .touchDragEnter)
        button.addTarget(self, action: #selector(self.startButtonUp), for: .touchDragExit)
        return button
    }()

    lazy var selectButton: UIButton = {
        let button = StartSelectButtonsView.createButton()
        button.addTarget(self, action: #selector(self.selectButtonDown), for: .touchDown)
        button.addTarget(self, action: #selector(self.selectButtonUp), for: .touchUpInside)
        button.addTarget(self, action: #selector(self.selectButtonDown), for: .touchDragEnter)
        button.addTarget(self, action: #selector(self.selectButtonUp), for: .touchDragExit)
        return button
    }()

    lazy var startImageView: UIImageView = {
        let image = UIImage(named: "start")
        let imageView = UIImageView(image: image)
        imageView.translatesAutoresizingMaskIntoConstraints = false
        return imageView
    }()

    lazy var selectImageView: UIImageView = {
        let image = UIImage(named: "select")
        let imageView = UIImageView(image: image)
        imageView.translatesAutoresizingMaskIntoConstraints = false
        return imageView
    }()

    required init?(coder aDecoder: NSCoder) {
        return nil
    }

    override init(frame: CGRect) {
        super.init(frame: frame)
        self.addSubview(self.startImageView)
        self.addSubview(self.selectImageView)
        self.addSubview(self.startButton)
        self.addSubview(self.selectButton)

        self.translatesAutoresizingMaskIntoConstraints = false

        self.selectImageView.trailingAnchor.constraint(equalTo: self.centerXAnchor).isActive = true
        self.selectImageView.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true
        self.selectImageView.heightAnchor.constraint(equalToConstant: StartSelectButtonsView.buttonImageSize.1).isActive = true
        self.selectImageView.widthAnchor.constraint(equalToConstant: StartSelectButtonsView.buttonImageSize.0).isActive = true

        self.startImageView.leadingAnchor.constraint(equalTo: self.centerXAnchor).isActive = true
        self.startImageView.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true
        self.startImageView.heightAnchor.constraint(equalToConstant: StartSelectButtonsView.buttonImageSize.1).isActive = true
        self.startImageView.widthAnchor.constraint(equalToConstant: StartSelectButtonsView.buttonImageSize.0).isActive = true

        self.selectButton.trailingAnchor.constraint(equalTo: self.centerXAnchor, constant: CGFloat(-2)).isActive = true
        self.selectButton.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true

        self.startButton.leadingAnchor.constraint(equalTo: self.centerXAnchor, constant: CGFloat(2)).isActive = true
        self.startButton.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true
    }

    convenience init() {
        self.init(frame: CGRect())
    }

    @objc func startButtonDown(sender: UIButton, event: UIControl.Event) {
        self.onButtonEvent?(.start, .down)
    }

    @objc func startButtonUp(sender: UIButton, event: UIControl.Event) {
        self.onButtonEvent?(.start, .up)
    }

    @objc func selectButtonDown(sender: UIButton, event: UIControl.Event) {
        self.onButtonEvent?(.select, .down)
    }

    @objc func selectButtonUp(sender: UIButton, event: UIControl.Event) {
        self.onButtonEvent?(.select, .up)
    }

    func onPan(points: [CGPoint], state: UIGestureRecognizer.State) {
        print("start")
    }

    class func createButton() -> UIButton {
        let button = UIButton()
        button.widthAnchor.constraint(equalToConstant: StartSelectButtonsView.buttonSize.0).isActive = true
        button.heightAnchor.constraint(equalToConstant: StartSelectButtonsView.buttonSize.1).isActive = true
        button.translatesAutoresizingMaskIntoConstraints = false
        return button
    }
}

