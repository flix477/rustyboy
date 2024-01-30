import SwiftUI
import SwiftData

struct HomeView: View {
    @Environment(\.modelContext) 
    private var modelContext
    
    @Query(sort: [SortDescriptor(\Game.lastPlayedDate), SortDescriptor(\Game.importDate)])
    private var games: [Game]
    
    @State
    private var isShowingFilePicker = false
    
    private let viewModel: HomeViewModel
    private let didSelectGame: (Game) -> Void
    
    init(viewModel: HomeViewModel,
         didSelectGame: @escaping (Game) -> Void) {
        self.viewModel = viewModel
        self.didSelectGame = didSelectGame
    }
    
    private func didSelectFiles(withResult result: Result<[URL], Error>) {
        switch result.mapError(HomeViewModel.ImportError.fileImporterError)
            .flatMap(viewModel.importGames(atURLs:)) {
        case .success(let newGames):
            withAnimation {
                for game in newGames {
                    modelContext.insert(game)
                }
            }
        case .failure(let failure):
            print(failure)
        }
    }
    
    private func didSelect(game: Game) {
        viewModel.didSelect(game: game)
        didSelectGame(game)
    }

    var body: some View {
        Group {
            if games.isEmpty {
                HomeEmptyStateView(didPressAddGame: { isShowingFilePicker = true })
            } else {
                GamesListView(didSelectGame: didSelect(game:),
                              didTapAddGame: { isShowingFilePicker = true },
                              groupGames: viewModel.group(games:))
            }
        }
        .fileImporter(isPresented: $isShowingFilePicker,
                      allowedContentTypes: [.data],
                      allowsMultipleSelection: true,
                      onCompletion: didSelectFiles(withResult:))
    }
}

#Preview {
    HomeView(viewModel: .init(now: Date.init), didSelectGame: { _ in })
        .modelContainer(for: [Game.self, Savestate.self], inMemory: true)
}
