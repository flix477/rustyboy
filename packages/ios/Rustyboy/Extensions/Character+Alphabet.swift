import Slingshot

extension Character {
    static var uppercaseAlphabet: [Character] {
        (65...90)
            .map(compose(UnicodeScalar.init, Character.init))
    }
}

extension Character: Identifiable {
    public var id: Self {
        self
    }
}
