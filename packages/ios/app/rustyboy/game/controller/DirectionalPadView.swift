import Foundation
import UIKit

class DirectionalPadView: UIView {
    static let margin = CGFloat(8)
    static let buttonSize = CGFloat(56)
    var pressed: ButtonType?

    lazy var panGestureRecognizer: UIPanGestureRecognizer = {
        let panGestureRecognizer = UIPanGestureRecognizer(target: self, action: #selector(self.onPan))
        return panGestureRecognizer
    }()

    override open class var requiresConstraintBasedLayout: Bool {
        return true
    }

    var onButtonEvent: ((ButtonType, ButtonEventType) -> Void)?

    lazy var downButton: UIButton = {
        let button = DirectionalPadView.createButton()
        button.addTarget(self, action: #selector(self.downButtonDown), for: .touchDown)
        button.addTarget(self, action: #selector(self.downButtonUp), for: .touchUpInside)
        return button
    }()

    lazy var upButton: UIButton = {
        let button = DirectionalPadView.createButton()
        button.addTarget(self, action: #selector(self.upButtonDown), for: .touchDown)
        button.addTarget(self, action: #selector(self.upButtonUp), for: .touchUpInside)
        return button
    }()

    lazy var leftButton: UIButton = {
        let button = DirectionalPadView.createButton()
        button.addTarget(self, action: #selector(self.leftButtonDown), for: .touchDown)
        button.addTarget(self, action: #selector(self.leftButtonUp), for: .touchUpInside)
        return button
    }()

    lazy var rightButton: UIButton = {
        let button = DirectionalPadView.createButton()
        button.addTarget(self, action: #selector(self.rightButtonDown), for: .touchDown)
        button.addTarget(self, action: #selector(self.rightButtonUp), for: .touchUpInside)
        return button
    }()

    lazy var imageView: UIImageView = {
        let image = UIImage(named: "dpad")
        let imageView = UIImageView(image: image)
        imageView.translatesAutoresizingMaskIntoConstraints = false
        return imageView
    }()

    required init?(coder aDecoder: NSCoder) {
        return nil
    }

    override init(frame: CGRect) {
        super.init(frame: frame)

        self.addGestureRecognizer(self.panGestureRecognizer)

        self.addSubview(self.imageView)
        self.addSubview(self.downButton)
        self.addSubview(self.upButton)
        self.addSubview(self.leftButton)
        self.addSubview(self.rightButton)

        self.translatesAutoresizingMaskIntoConstraints = false

        self.downButton.centerXAnchor.constraint(equalTo: self.centerXAnchor).isActive = true
        self.downButton.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true

        self.upButton.centerXAnchor.constraint(equalTo: self.centerXAnchor).isActive = true
        self.upButton.topAnchor.constraint(equalTo: self.topAnchor).isActive = true

        self.leftButton.centerYAnchor.constraint(equalTo: self.centerYAnchor).isActive = true
        self.leftButton.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true

        self.rightButton.centerYAnchor.constraint(equalTo: self.centerYAnchor).isActive = true
        self.rightButton.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true

        self.imageView.leadingAnchor.constraint(
            equalTo: self.leadingAnchor,
            constant: DirectionalPadView.margin
        ).isActive = true
        self.imageView.trailingAnchor.constraint(
            equalTo: self.trailingAnchor,
            constant: -DirectionalPadView.margin
        ).isActive = true
        self.imageView.topAnchor.constraint(
            equalTo: self.topAnchor,
            constant: DirectionalPadView.margin
        ).isActive = true
        self.imageView.bottomAnchor.constraint(
            equalTo: self.bottomAnchor,
            constant: -DirectionalPadView.margin
        ).isActive = true
    }

    convenience init() {
        self.init(frame: CGRect())
    }

    @objc func downButtonDown(sender: UIButton, event: UIControl.Event) {
        self.pressed = .down
        self.onButtonEvent?(.down, .down)
    }

    @objc func downButtonUp(sender: UIButton, event: UIControl.Event) {
        self.pressed = nil
        self.onButtonEvent?(.down, .up)
    }

    @objc func upButtonDown(sender: UIButton, event: UIControl.Event) {
        self.pressed = .up
        self.onButtonEvent?(.up, .down)
    }

    @objc func upButtonUp(sender: UIButton, event: UIControl.Event) {
        self.pressed = nil
        self.onButtonEvent?(.up, .up)
    }

    @objc func leftButtonDown(sender: UIButton, event: UIControl.Event) {
        self.pressed = .left
        self.onButtonEvent?(.left, .down)
    }

    @objc func leftButtonUp(sender: UIButton, event: UIControl.Event) {
        self.pressed = nil
        self.onButtonEvent?(.left, .up)
    }

    @objc func rightButtonDown(sender: UIButton, event: UIControl.Event) {
        self.pressed = .right
        self.onButtonEvent?(.right, .down)
    }

    @objc func rightButtonUp(sender: UIButton, event: UIControl.Event) {
        self.pressed = nil
        self.onButtonEvent?(.right, .up)
    }

    class func createButton() -> UIButton {
        let button = UIButton()
        button.widthAnchor.constraint(equalToConstant: DirectionalPadView.buttonSize).isActive = true
        button.heightAnchor.constraint(equalToConstant: DirectionalPadView.buttonSize).isActive = true
        button.translatesAutoresizingMaskIntoConstraints = false
        return button
    }

    func buttons() -> [UIButton] {
        return [self.downButton, self.upButton, self.rightButton, self.leftButton]
    }

    @objc func onPan() {
        let point = self.panGestureRecognizer.location(in: self)
        let state = self.panGestureRecognizer.state
        guard let button = self.hitTest(point, with: nil) as? UIButton else {
            if let pressed = self.pressed {
                self.onButtonEvent?(pressed, .up)
                self.pressed = nil
            }
            return
        }
        let buttonType = self.buttonType(button)!

        if state == .began || state == .changed {
            if let pressed = self.pressed {
                if pressed != buttonType {
                    self.onButtonEvent?(pressed, .up)
                    self.pressed = buttonType
                    self.onButtonEvent?(buttonType, .down)
                }
            } else {
                self.pressed = buttonType
                self.onButtonEvent?(buttonType, .down)
            }
        } else if state == .ended {
            if let pressed = self.pressed {
                self.onButtonEvent?(pressed, .up)
                self.pressed = nil
            }
        }
    }

    func buttonType(_ button: UIButton) -> ButtonType? {
        switch button {
        case self.downButton:
            return .down
        case self.upButton:
            return .up
        case self.leftButton:
            return .left
        case self.rightButton:
            return .right
        default:
            return nil
        }
    }
}
