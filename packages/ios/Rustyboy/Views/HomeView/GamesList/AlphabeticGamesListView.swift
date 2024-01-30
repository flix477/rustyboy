import SwiftUI
import SwiftData

struct AlphabeticGamesListView: View {
    @Query(sort: \Game.name)
    private var games: [Game]
    
    let proxy: ScrollViewProxy
    let didSelectGame: (Game) -> Void
    
    @State
    private var selectedLetter: Character = "A"
    
    private var availableLetters: Set<Character> {
        Set(games.compactMap(\.name.first))
    }
    
    private func first(gameStartingWith character: Character) -> Game? {
        games.first(where: { $0.name.first == character })
    }
    
    var body: some View {
        HStack(spacing: 0) {
            ScrollView {
                LazyVGrid(columns: [.init(), .init()], spacing: 24, pinnedViews: .sectionHeaders) {
                    ForEach(games) { game in
                        GameListCellView(name: game.name, didSelect: { didSelectGame(game) })
                            .id(game.id)
                    }
                }
                .padding(.horizontal, 16)
            }
            .onChange(of: selectedLetter) { _, newValue in
                guard let game = first(gameStartingWith: newValue) else { return }
                proxy.scrollTo(game.id, anchor: .center)
            }
            .frame(maxWidth: .infinity)
            
            AlphabetScrollIndicatorView(selection: $selectedLetter,
                                        availableLetters: availableLetters)
        }
    }
}
