//
//  HomeView.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-11.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import SwiftUI
import UniformTypeIdentifiers

struct HomeView: View {
    let controller = HomeController()
    @State var showFilePicker = false
    @State var gameboy: Gameboy?

    var body: some View {
        if let gameboy = self.gameboy {
            return AnyView(GameView(gameboy: gameboy))
        }

        return AnyView(VStack(spacing: 28) {
            Text("RUSTY BOY")
                .font(.semiBoldItalic(56))
                .foregroundColor(.init(red: 77, green: 78, blue: 141))

            VStack(spacing: 8) {
                Button(action: { showFilePicker = true }, label: {
                    Text("Load game")
                        .font(.semiBold(24))
                        .foregroundColor(.white)
                })
                .cornerRadius(5)
                .padding(EdgeInsets(top: 24, leading: 64, bottom: 24, trailing: 64))
                .background(Color.tint)

                Button(action: {}, label: {
                    Text("Options")
                        .font(.semiBold(18))
                        .foregroundColor(.init(red: 45, green: 45, blue: 45))
                })
                .padding(EdgeInsets(top: 16, leading: 64, bottom: 16, trailing: 64))
            }
        }.sheet(isPresented: $showFilePicker) {
            DocumentPicker { urls in
                guard let url = urls.first else { return }
                controller.onFileSelection(path: url).unsafeRunAsync(on: .global()) { result in
                    switch result.toResult() {
                    case .success(let gameboy):
                        self.gameboy = gameboy
                    case .failure(let error):
                        print(error)
                    }
                }
            }
        }
        .background(Color.white))
    }
}

struct HomeViewPreviews: PreviewProvider {
    static var previews: some View {
        HomeView()
    }
}
