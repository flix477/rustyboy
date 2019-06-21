import UIKit

class HomeViewController: UIViewController, UIDocumentPickerDelegate {
    var homeController = HomeController()

    // TODO: find out why it's clipped
    lazy var titleLabel: UILabel = {
        let label = UILabel()
        label.font = UIFont(name: "Cabin-SemiBoldItalic", size: 56 )!
        let attributedTitle = NSAttributedString(
            string: "GAME BOY",
            attributes: [
                NSAttributedString.Key.kern: -3,
                NSAttributedString.Key.foregroundColor: UIColor.init(red: 77, green: 78, blue: 141)
            ]
        )
        label.attributedText = attributedTitle
        label.sizeToFit()

        return label
    }()

    lazy var loadFileButton: UIButton = {
        let button = UIButton(type: .custom)
        button.backgroundColor = UIColor.init(red: 169, green: 60, blue: 111)
        button.setTitleColor(UIColor.white, for: .normal)
        button.setTitleColor(UIColor.lightGray, for: .highlighted)
        button.setTitle("Load game", for: .normal)
        button.titleLabel?.font = UIFont(name: "Cabin-SemiBold", size: 24)
        button.contentEdgeInsets = UIEdgeInsets(top: 24, left: 64, bottom: 24, right: 64)
        button.layer.cornerRadius = 5
        button.addTarget(self, action: #selector(self.loadFileButtonPressed), for: .touchUpInside)

        return button
    }()

    lazy var optionsButton: UIButton = {
        let button = UIButton(type: .custom)
        button.setTitleColor(UIColor.init(red: 45, green: 45, blue: 45), for: .normal)
        button.setTitleColor(UIColor.lightGray, for: .highlighted)
        button.setTitle("Options", for: .normal)
        button.titleLabel?.font = UIFont(name: "Cabin-SemiBold", size: 18)
        button.contentEdgeInsets = UIEdgeInsets(top: 16, left: 64, bottom: 16, right: 64)

        return button
    }()

    lazy var buttonsStackView: UIStackView = {
        let stack = UIStackView(arrangedSubviews: [self.loadFileButton, self.optionsButton])
        stack.axis = .vertical
        stack.alignment = .center
        stack.distribution = .equalSpacing
        stack.spacing = 8
        stack.translatesAutoresizingMaskIntoConstraints = false

        return stack
    }()

    lazy var containerStackView: UIStackView = {
        let stack = UIStackView(arrangedSubviews: [self.titleLabel, self.buttonsStackView])
        stack.axis = .vertical
        stack.alignment = .center
        stack.distribution = .equalSpacing
        stack.spacing = 28
        stack.translatesAutoresizingMaskIntoConstraints = false

        return stack
    }()

    override var preferredStatusBarStyle: UIStatusBarStyle {
        return .default
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        self.view.backgroundColor = UIColor.white
        self.view.addSubview(self.containerStackView)
        self.setupStackView()
    }

    func setupStackView() {
        self.containerStackView.centerXAnchor.constraint(equalTo: self.view.centerXAnchor).isActive = true
        self.containerStackView.centerYAnchor.constraint(equalTo: self.view.centerYAnchor).isActive = true
    }

    @objc func loadFileButtonPressed(sender: UIButton) {
        let picker = UIDocumentPickerViewController(documentTypes: ["com.fleveille.rustyboy.gb"], in: .import)
        picker.delegate = self
        picker.modalPresentationStyle = .fullScreen
        self.present(picker, animated: true)
    }

    func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
        switch self.homeController.onFileSelection(path: urls[0]) {
            case .success(let gameboy):
                let gameViewController = GameViewController()
                gameViewController.gameboy = gameboy
                self.present(gameViewController, animated: true)
            case .failure(let error):
                self.displayError(error)
        }
    }

    func displayError(_ error: GameLoadError) {
        var errorString = ""
        switch error {
        case .errorOpeningFile:
            errorString = "An error occured while opening this file"
        case .notAGameboyROM:
            errorString = "This file does not appear to be a valid GameBoy ROM"
        }

        let alert = UIAlertController.init(title: "Error loading game", message: errorString, preferredStyle: .alert)
        alert.addAction(UIAlertAction(title: "OK", style: .default, handler: nil))
        self.present(alert, animated: true)
    }
}

extension UIColor {
    convenience init(red: CGFloat, green: CGFloat, blue: CGFloat) {
        self.init(red: red / 255, green: green / 255, blue: blue / 255, alpha: 1.0)
    }
}
