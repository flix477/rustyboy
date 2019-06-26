import UIKit

class HomeViewController: UIViewController, UIDocumentPickerDelegate {
    let homeController = HomeController()

    private var _loading = false
    var loading: Bool {
        get { return self._loading }
        set {
            self._loading = newValue
            self.loadFileButton.isEnabled = !newValue
            self.optionsButton.isEnabled = !newValue
        }
    }

    override var preferredStatusBarStyle: UIStatusBarStyle {
        return .default
    }

    // TODO: find out why it's clipped
    lazy var titleLabel: UILabel = {
        let label = UILabel()
        label.font = UIFont(name: Theme.fontFamily.semiBoldItalic, size: 56 )!
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
        button.backgroundColor = Theme.tintColor
        button.setTitleColor(UIColor.white, for: .normal)
        button.setTitleColor(UIColor.lightGray, for: .highlighted)
        button.setTitle("Load game", for: .normal)
        button.titleLabel?.font = UIFont(name: Theme.fontFamily.semiBold, size: 24)
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
        button.titleLabel?.font = UIFont(name: Theme.fontFamily.semiBold, size: 18)
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

    override func viewDidLoad() {
        super.viewDidLoad()
        self.view.backgroundColor = UIColor.white
        self.view.addSubview(self.containerStackView)
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
        self.loading = true
        self.homeController.onFileSelection(path: urls[0]) { result in
            self.loading = false
            switch result {
                case .success(let gameboy):
                    let gameViewController = GameViewController()
                    gameViewController.gameboy = gameboy
                    self.present(gameViewController, animated: true)
                case .failure(let error):
                    self.displayError(error)
            }
        }
    }

    func displayError(_ error: GameLoadError) {
        let errorString = self.homeController.errorToString(error)
        let alert = UIAlertController.init(title: "Error loading game", message: errorString, preferredStyle: .alert)
        alert.addAction(UIAlertAction(title: "OK", style: .default, handler: nil))
        self.present(alert, animated: true)
    }
}
