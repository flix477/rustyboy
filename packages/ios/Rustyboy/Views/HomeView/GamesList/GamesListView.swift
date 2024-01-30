import SwiftUI
import SwiftData
import Slingshot

struct GamesListView: View {
    enum Sort: String, CaseIterable, Identifiable {
        case lastPlayed, alphabetical
        
        var id: String { rawValue }
        
        var symbol: String {
            switch self {
            case .lastPlayed:
                "clock"
            case .alphabetical:
                "character"
            }
        }
    }
    
    @State
    private var selectedSort: Sort = .lastPlayed
    private let didSelectGame: (Game) -> Void
    private let didTapAddGame: () -> Void
    private let groupGames: ([Game]) -> [(String, [Game])]
    
    init(didSelectGame: @escaping (Game) -> Void,
         didTapAddGame: @escaping () -> Void,
         groupGames: @escaping ([Game]) -> [(String, [Game])]) {
        self.didSelectGame = didSelectGame
        self.didTapAddGame = didTapAddGame
        self.groupGames = groupGames
    }
    
    var body: some View {
        NavigationStack {
            ScrollViewReader { proxy in
                if selectedSort == .alphabetical {
                    AlphabeticGamesListView(proxy: proxy, didSelectGame: didSelectGame)
                        .padding(.top, 16)
                } else {
                    RecentlyPlayedGamesListView(didSelectGame: didSelectGame, groupGames: groupGames)
                }
            }
            .animation(.default, value: selectedSort)
            .frame(maxWidth: .infinity)
            .toolbar {
                Picker("Sort", selection: $selectedSort) {
                    ForEach(Sort.allCases) { sort in
                        Image(systemName: sort.symbol)
                            .tag(sort)
                    }
                }
                .pickerStyle(.segmented)
                
                Button(action: didTapAddGame) {
                    Image(systemName: "plus")
                }
            }
            .navigationTitle("games")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

#Preview {
    let config = ModelConfiguration(isStoredInMemoryOnly: true)
    let container = try! ModelContainer(for: Game.self, Savestate.self,
                                        configurations: config)
    
    container.mainContext.insert(Game(name: "Merio 2", rom: .init(), importDate: .now))
    container.mainContext.insert(Game(name: "Tertis", rom: .init(), importDate: .now))
    container.mainContext.insert(Game(name: "Zeldo", rom: .init(), importDate: .now))
    container.mainContext.insert(Game(name: "Zeldo 3", rom: .init(), importDate: .now))
    container.mainContext.insert(Game(name: "Merio", rom: .init(), importDate: .now))
    container.mainContext.insert(Game(name: "Pokemen", rom: .init(), importDate: .now))
    container.mainContext.insert(Game(name: "Pokemen Orange", rom: .init(), importDate: .now))
    
    return GamesListView(didSelectGame: { _ in },
                         didTapAddGame: {},
                         groupGames: { games in HomeViewModel(now: Date.init).group(games: games) })
        .modelContainer(container)
}
