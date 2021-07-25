//
//  HomeView.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-11.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import SwiftUI
import UniformTypeIdentifiers

struct HomeView<D>: View where D: HasPersistence {
    let environment: D
    @State private var loading: Bool = true
    @State private var showFilePicker = false
    @State private var games: [Game] = []

    private func fetchGames() {
        self.loading = true
        HomeController.fetchGames().unsafeRunAsync(with: environment.persistence, on: .main) { result in
            switch result.toResult() {
            case .success(let games):
                self.games = games
            case .failure(let error):
                print(error)
            }

            self.loading = false
        }
    }

    private func importGames(urls: [URL]) {
        guard let url = urls.first else { return }
        HomeController.importGame(at: url).unsafeRunAsync(with: environment.persistence, on: .main) { result in
            switch result.toResult() {
            case .success:
                self.fetchGames()
            case .failure(let error):
                print(error)
            }
        }
    }

    var body: some View {
        VStack {
            if loading {
                ProgressView()
            } else if games.isEmpty {
                HomeEmptyStateView { showFilePicker = true }
            } else {
                GameListView(environment: environment, games: games) { showFilePicker = true }
            }
        }
        .sheet(isPresented: $showFilePicker) {
            DocumentPicker(didSelectURLs: importGames)
        }
        .onAppear(perform: fetchGames)
    }
}

struct HomeViewPreviews: PreviewProvider {
    private struct Environment: HasPersistence {
        var persistence: some Persistence { return MockPersistence() }
    }

    static var previews: some View {
        HomeView(environment: Environment())
    }
}
