import SwiftUI
import SwiftData

struct RecentlyPlayedGamesListView: View {
    let didSelectGame: (Game) -> Void
    let groupGames: ([Game]) -> [(String, [Game])]
    
    @Query(sort: [SortDescriptor(\Game.lastPlayedDate, order: .reverse),
                  SortDescriptor(\Game.importDate, order: .reverse)])
    private var sortedGames: [Game]
    
    private var games: [(String, [Game])] {
        groupGames(sortedGames)
    }
    
    var body: some View {
        ScrollView {
            LazyVGrid(columns: [.init(), .init()], spacing: 24, pinnedViews: .sectionHeaders) {
                ForEach(games, id: \.0) { section in
                    Section {
                        ForEach(section.1) { game in
                            GameListCellView(name: game.name, didSelect: { didSelectGame(game) })
                        }
                    } header: {
                        HStack {
                            Text(section.0.localized)
                                .font(.semiBold(20))
                            
                            Spacer()
                        }
                        .padding()
                        .background(.thinMaterial)
                        .clipShape(RoundedRectangle(cornerRadius: 26))
                        .padding(.top, 8)
                    }
                    .headerProminence(.standard)
                    .id(section.0)
                }
            }
            .padding(.horizontal, 16)
        }
    }
}
