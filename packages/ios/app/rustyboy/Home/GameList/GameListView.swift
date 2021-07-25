//
//  GameListView.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-18.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation

import SwiftUI
import UniformTypeIdentifiers

struct EmulatorEnvironment<P> where P: Persistence {
    private let _persistence: P
    private let _game: Game
    private let _gameboy: Gameboy

    init(persistence: P, game: Game, gameboy: Gameboy) {
        self._persistence = persistence
        self._game = game
        self._gameboy = gameboy
    }
}

extension EmulatorEnvironment: HasPersistence, HasGameboy {
    var persistence: some Persistence { return _persistence }
    var game: Game { return _game }
    var gameboy: Gameboy { return _gameboy }
}

struct GameListView<D>: View where D: HasPersistence {
    let environment: D
    let games: [Game]
    let addGamePressed: () -> Void
    @State private var currentlyPlaying: (Gameboy, Game)?

    private func onGameTap(game id: String) -> () -> Void {
        let game = games.first { $0._id.stringValue == id }!
        return {
            HomeController.loadGame(game: game).unsafeRunAsync(with: environment, on: .main) { result in
                switch result.toResult() {
                case .success(let gameboy):
                    self.currentlyPlaying = (gameboy, game)
                case .failure(let error):
                    print(error)
                }
            }
        }
    }

    var body: some View {
        if let (gameboy, game) = self.currentlyPlaying {
            return AnyView(EmulatorView(environment: EmulatorEnvironment(persistence: environment.persistence,
                                                                         game: game,
                                                                         gameboy: gameboy),
                                        quit: { currentlyPlaying = nil }))
        }

        return AnyView(
            GameListDumbView(games: games.map(GameViewModel.from),
                             addGamePressed: addGamePressed,
                             onGameTap: onGameTap)
        )
    }
}

private struct GameListDumbView: View {
    let games: [GameViewModel]
    let addGamePressed: () -> Void
    let onGameTap: (String) -> () -> Void

    let columns = [
        GridItem(.adaptive(minimum: 160))
    ]

    var body: some View {
        ScrollView(.vertical, showsIndicators: false) {
            VStack(alignment: .leading, spacing: 32) {
                HStack {
                    Text("Games").font(.semiBold(36))
                    Spacer()
                    Button(action: addGamePressed, label: {
                        Image("add_white")
                            .resizable()
                            .aspectRatio(contentMode: .fit)
                            .frame(width: 42, height: 42)
                            .opacity(0.8)
                    })
                }

                LazyVGrid(columns: columns, spacing: 32) {
                    ForEach(games, id: \.id) { game in
                        GameListCellView(model: game)
                            .onTapGesture(perform: onGameTap(game.id))
                    }
                }
            }
        }
        .padding(24)
        .foregroundColor(.white)
        .background(LinearGradient(gradient: .primary,
                                   startPoint: .topLeading,
                                   endPoint: .bottomTrailing).edgesIgnoringSafeArea(.all))
    }
}

struct GameListViewPreviews: PreviewProvider {
    static var previews: some View {
        GameListDumbView(games: [GameViewModel(id: "1", name: "Super Mario Land"),
                                 GameViewModel(id: "2", name: "Tetris"),
                                 GameViewModel(id: "3", name: "Zelda")],
                         addGamePressed: {},
                         onGameTap: { _ in {} })
    }
}
