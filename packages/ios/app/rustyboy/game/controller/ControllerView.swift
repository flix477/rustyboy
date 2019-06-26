import Foundation
import UIKit

class ControllerView: UIView {
    static let directionalPadSize = CGFloat(168)
    static let abButtonsSize = (CGFloat(180), CGFloat(140))
    static let startSelectSize = (CGFloat(180), CGFloat(56))
    static let margin = CGFloat(8)
    static let spacing = CGFloat(38)
    var onButtonEvent: ((ButtonType, ButtonEventType) -> ())?

    lazy var directionalPad: DirectionalPadView = {
        let directionalPad = DirectionalPadView()
        directionalPad.widthAnchor.constraint(equalToConstant: ControllerView.directionalPadSize).isActive = true
        directionalPad.heightAnchor.constraint(equalToConstant: ControllerView.directionalPadSize).isActive = true
        directionalPad.onButtonEvent = { button, event in
            self.onButtonEvent?(button, event)
        }
        return directionalPad
    }()

    lazy var abButtons: ABButtonsView = {
        let abButtons = ABButtonsView()
        abButtons.widthAnchor.constraint(equalToConstant: ControllerView.abButtonsSize.0).isActive = true
        abButtons.heightAnchor.constraint(equalToConstant: ControllerView.abButtonsSize.1).isActive = true
        abButtons.onButtonEvent = { button, event in
            self.onButtonEvent?(button, event)
        }
        return abButtons
    }()

    lazy var startSelectButtons: StartSelectButtonsView = {
        let startSelectButtons = StartSelectButtonsView()
        startSelectButtons.widthAnchor.constraint(equalToConstant: ControllerView.startSelectSize.0).isActive = true
        startSelectButtons.heightAnchor.constraint(equalToConstant: ControllerView.startSelectSize.1).isActive = true
        startSelectButtons.onButtonEvent = { button, event in
            self.onButtonEvent?(button, event)
        }
        return startSelectButtons
    }()

    override open class var requiresConstraintBasedLayout: Bool {
        return true
    }

    required init?(coder aDecoder: NSCoder) {
        return nil
    }

    override init(frame: CGRect) {
        super.init(frame: frame)
        self.translatesAutoresizingMaskIntoConstraints = false
        self.addSubview(self.directionalPad)
        self.addSubview(self.abButtons)
        self.addSubview(self.startSelectButtons)
        self.backgroundColor = UIColor(red: 240, green: 240, blue: 240)

        self.startSelectButtons.centerXAnchor.constraint(equalTo: self.centerXAnchor).isActive = true
        self.startSelectButtons.bottomAnchor.constraint(
            equalTo: self.bottomAnchor,
            constant: -ControllerView.margin
        ).isActive = true

        self.directionalPad.leadingAnchor.constraint(
            equalTo: self.leadingAnchor,
            constant: ControllerView.margin
        ).isActive = true
        self.directionalPad.bottomAnchor.constraint(
            equalTo: self.startSelectButtons.topAnchor,
            constant: -ControllerView.margin * 4
        ).isActive = true

        self.abButtons.trailingAnchor.constraint(
            equalTo: self.trailingAnchor,
            constant: -ControllerView.margin
        ).isActive = true
        self.abButtons.centerYAnchor.constraint(equalTo: self.directionalPad.centerYAnchor).isActive = true

    }
}
