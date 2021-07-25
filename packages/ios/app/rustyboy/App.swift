import SwiftUI

@main
struct RustyboyApp: App {
    private struct Environment: HasPersistence {
        var persistence: some Persistence {
            return RealmPersistence()
        }
    }

    var body: some Scene {
        WindowGroup {
            HomeView(environment: Environment())
        }
    }
}
