import Foundation
import UIKit

class ABButtonsView: UIView {
    static let buttonSize = CGFloat(84)
    var pressed: ButtonType?

    lazy var panGestureRecognizer: UIPanGestureRecognizer = {
        let panGestureRecognizer = UIPanGestureRecognizer(target: self, action: #selector(self.onPan))
        return panGestureRecognizer
    }()

    override open class var requiresConstraintBasedLayout: Bool {
        return true
    }

    var onButtonEvent: ((ButtonType, ButtonEventType) -> ())?

    lazy var aButton: UIButton = {
        let button = ABButtonsView.createButton()
        button.addTarget(self, action: #selector(self.aButtonDown), for: .touchDown)
        button.addTarget(self, action: #selector(self.aButtonUp), for: .touchUpInside)
        button.addTarget(self, action: #selector(self.aButtonDown), for: .touchDragEnter)
        button.addTarget(self, action: #selector(self.aButtonUp), for: .touchDragExit)
        return button
    }()

    lazy var bButton: UIButton = {
        let button = ABButtonsView.createButton()
        button.addTarget(self, action: #selector(self.bButtonDown), for: .touchDown)
        button.addTarget(self, action: #selector(self.bButtonUp), for: .touchUpInside)
        button.addTarget(self, action: #selector(self.bButtonDown), for: .touchDragEnter)
        button.addTarget(self, action: #selector(self.bButtonUp), for: .touchDragExit)
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

        self.aButton.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
        self.aButton.topAnchor.constraint(equalTo: self.topAnchor).isActive = true

        self.bButton.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true
        self.bButton.bottomAnchor.constraint(equalTo: self.bottomAnchor, constant: CGFloat(-24)).isActive = true

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

    @objc func aButtonDown(sender: UIButton, event: UIControl.Event) {
        self.pressed = .a
        self.onButtonEvent?(.a, .down)
    }

    @objc func aButtonUp(sender: UIButton, event: UIControl.Event) {
        self.pressed = nil
        self.onButtonEvent?(.a, .up)
    }

    @objc func bButtonDown(sender: UIButton, event: UIControl.Event) {
        self.pressed = .b
        self.onButtonEvent?(.b, .down)
    }

    @objc func bButtonUp(sender: UIButton, event: UIControl.Event) {
        self.pressed = nil
        self.onButtonEvent?(.b, .up)
    }

    @objc func onPan() {
        let point = self.panGestureRecognizer.location(in: self)
        let state = self.panGestureRecognizer.state
        guard let button = self.hitTest(point, with: nil) as? UIButton else {
            if let pressed = self.pressed {
                print(state.toString())
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
        case self.aButton:
            return .a
        case self.bButton:
            return .b
        default:
            return nil
        }
    }

    class func createButton() -> UIButton {
        let button = UIButton()
        button.widthAnchor.constraint(equalToConstant: ABButtonsView.buttonSize).isActive = true
        button.heightAnchor.constraint(equalToConstant: ABButtonsView.buttonSize).isActive = true
        button.translatesAutoresizingMaskIntoConstraints = false
        return button
    }
}

