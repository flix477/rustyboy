import SwiftUI

struct AlphabetScrollIndicatorView: View {
    private struct Constants {
        static let spacing: CGFloat = 8
        static let letterHeight: CGFloat = 12
    }
    
    @State
    private var isDragging = false
    
    @Binding
    private var selection: Character
    
    let availableLetters: Set<Character>
    
    init(selection: Binding<Character>, availableLetters: Set<Character>) {
        self._selection = selection
        self.availableLetters = availableLetters
    }
    
    private var gesture: some Gesture {
        DragGesture(minimumDistance: 0)
            .onChanged { value in
                isDragging = true
                let index = min(UInt8(max(value.location.y - 4, 0) / (Constants.letterHeight + Constants.spacing / 2)), 25)
                let character = Character(UnicodeScalar(index + 65))
                if availableLetters.contains(character) {
                    selection = character
                }
            }
            .onEnded { _ in isDragging = false }
    }
    
    var body: some View {
        VStack(spacing: Constants.spacing) {
            ForEach(Character.uppercaseAlphabet) { (letter: Character) in
                let isAvailable = availableLetters.contains(letter)
                
                Text(isAvailable ? String(letter) : "·")
                    .font(.regular(12))
                    .foregroundStyle(.accent)
                    .frame(width: 15, height: Constants.letterHeight)
            }
        }
        .padding(.horizontal, 4)
        .padding(.vertical, 8)
        .sensoryFeedback(.selection, trigger: selection)
        .background(.thinMaterial.opacity(isDragging ? 1.0 : 0))
        .gesture(gesture)
        .animation(.default, value: isDragging)
        .clipShape(RoundedRectangle(cornerRadius: 23))
    }
}

#Preview {
    AlphabetScrollIndicatorView(selection: .constant("A"),
                                availableLetters: Set(Character.uppercaseAlphabet))
}
