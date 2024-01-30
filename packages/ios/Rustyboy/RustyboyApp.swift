import SwiftUI
import SwiftData

@main
struct RustyboyApp: App {
    var sharedModelContainer: ModelContainer = {
        let schema = Schema([
            Game.self,
            Savestate.self
        ])
        let modelConfiguration = ModelConfiguration(schema: schema, isStoredInMemoryOnly: false)

        do {
            return try ModelContainer(for: schema, configurations: [modelConfiguration])
        } catch {
            fatalError("Could not create ModelContainer: \(error)")
        }
    }()
    
    @State
    private var selectedGame: Game?

    var body: some Scene {
        WindowGroup {
            HomeView(viewModel: .init(now: Date.init),
                     didSelectGame: { selectedGame = $0 })
            .fullScreenCover(item: $selectedGame) { game in
                GameView(viewModel: .init(game: game)) { selectedGame = nil }
            }
        }
        .modelContainer(sharedModelContainer)
    }
}
