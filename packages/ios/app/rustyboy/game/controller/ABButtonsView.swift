import Foundation
import UIKit

class ABButtonsView: UIView {
    static let buttonSize = CGFloat(70)
    static let aButtonOffset = (CGFloat(-10), CGFloat(6))
    static let bButtonOffset = (CGFloat(10), CGFloat(-31))
    var pressed: GameboyButtonType?

    lazy var panGestureRecognizer: UIPanGestureRecognizer = {
        let panGestureRecognizer = UIPanGestureRecognizer(target: self, action: #selector(self.onPan))
        return panGestureRecognizer
    }()

    override open class var requiresConstraintBasedLayout: Bool {
        return true
    }

    var onButtonEvent: ((GameboyButtonType, ButtonEventType) -> Void)?

    lazy var aButton: UIButton = {
        let button = ABButtonsView.createButton(type: .a)
        button.onButtonEvent = self.onABButtonEvent
        return button
    }()

    lazy var bButton: UIButton = {
        let button = ABButtonsView.createButton(type: .b)
        button.onButtonEvent = self.onABButtonEvent
        return button
    }()

    lazy var imageView: UIImageView = {
        let image = UIImage(named: "abbuttons")
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
        self.addSubview(self.aButton)
        self.addSubview(self.bButton)
        self.addSubview(self.imageView)

        self.translatesAutoresizingMaskIntoConstraints = false

        self.aButton.trailingAnchor.constraint(
            equalTo: self.trailingAnchor,
            constant: ABButtonsView.aButtonOffset.0
        ).isActive = true
        self.aButton.topAnchor.constraint(
            equalTo: self.topAnchor,
            constant: ABButtonsView.aButtonOffset.1
        ).isActive = true

        self.bButton.leadingAnchor.constraint(
            equalTo: self.leadingAnchor,
            constant: ABButtonsView.bButtonOffset.0
        ).isActive = true
        self.bButton.bottomAnchor.constraint(
            equalTo: self.bottomAnchor,
            constant: ABButtonsView.bButtonOffset.1
        ).isActive = true

        self.imageView.leadingAnchor.constraint(
            equalTo: self.leadingAnchor
        ).isActive = true
        self.imageView.trailingAnchor.constraint(
            equalTo: self.trailingAnchor
        ).isActive = true
        self.imageView.topAnchor.constraint(
            equalTo: self.topAnchor
        ).isActive = true
        self.imageView.bottomAnchor.constraint(
            equalTo: self.bottomAnchor
        ).isActive = true
    }

    convenience init() {
        self.init(frame: CGRect())
    }

    func onABButtonEvent(buttonType: GameboyButtonType, eventType: ButtonEventType) {
        if eventType == .up {
            self.pressed = nil
        } else {
            self.pressed = buttonType
        }

        self.onButtonEvent?(buttonType, eventType)
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

    func buttonType(_ button: UIButton) -> GameboyButtonType? {
        switch button {
        case self.aButton:
            return .a
        case self.bButton:
            return .b
        default:
            return nil
        }
    }

    class func createButton(type: GameboyButtonType) -> ABButton {
        let button = ABButton(type: type)
        button.widthAnchor.constraint(equalToConstant: ABButtonsView.buttonSize).isActive = true
        button.heightAnchor.constraint(equalToConstant: ABButtonsView.buttonSize).isActive = true
        button.translatesAutoresizingMaskIntoConstraints = false
        return button
    }
}

class ABButton: UIButton {
    let gameboyButtonType: GameboyButtonType

    let image = UIImage(named: "abbuttons-button")!
    let pressedImage = UIImage(named: "abbuttons-button-pressed")!

    var onButtonEvent: ((GameboyButtonType, ButtonEventType) -> Void)?
    var padding = (CGFloat(16), CGFloat(16))

    init(type: GameboyButtonType) {
        self.gameboyButtonType = type
        super.init(frame: CGRect.zero)

        self.setImage(self.image, for: .normal)
        self.setImage(self.pressedImage, for: .highlighted)

        self.addTarget(self, action: #selector(self.buttonDown), for: .touchDown)
        self.addTarget(self, action: #selector(self.buttonUp), for: .touchUpInside)
        self.addTarget(self, action: #selector(self.buttonDown), for: .touchDragEnter)
        self.addTarget(self, action: #selector(self.buttonUp), for: .touchDragExit)
    }

    override func point(inside point: CGPoint, with event: UIEvent?) -> Bool {
        let newArea = CGRect(
            x: self.bounds.origin.x - self.padding.0,
            y: self.bounds.origin.y - self.padding.1,
            width: self.bounds.size.width + self.padding.0 * 2,
            height: self.bounds.size.height + self.padding.1 * 2
        )
        return newArea.contains(point)
    }

    required init?(coder aDecoder: NSCoder) {
        return nil
    }

    @objc func buttonDown(sender: UIButton, event: UIControl.Event) {
        self.onButtonEvent?(self.gameboyButtonType, .down)
    }

    @objc func buttonUp(sender: UIButton, event: UIControl.Event) {
        self.onButtonEvent?(self.gameboyButtonType, .up)
    }
}
