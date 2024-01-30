//
//  GameMenuView.swift
//  Rustyboy
//
//  Created by Félix Léveillé on 2024-01-12.
//

import SwiftUI
import SwiftData

struct GameMenuView: View {
    let sendInput: (Input) -> Void
    
    @Query
    private let _savestates: [Savestate]
    private let game: Game
    
    @Environment(\.modelContext)
    private var modelContext
    
    enum Input {
        case didTapSavestate(Savestate)
        case didTapReset
        case didTapSave
        case didTapDismiss
        case didTapExit
    }
    
    private var savestates: [Savestate] {
        _savestates.filter { $0.game == game }
    }
    
    init(sendInput: @escaping (Input) -> Void, game: Game) {
        self.sendInput = sendInput
        self.game = game
    }
    
    private func action(name: String.LocalizationValue, input: Input, icon: String, offset: CGFloat = 0) -> some View {
        Button(action: { sendInput(input) }) {
            VStack {
                Image(systemName: icon)
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(width: 48 + offset, height: 48 + offset)
                Text(String(localized: name))
                    .font(.semiBold(14))
            }
        }
    }
    
    private var cardBackground: some View {
        RoundedRectangle(cornerRadius: 24)
            .fill(.white.opacity(0.1))
            .shadow(color: .black.opacity(0.1), radius: 8)
    }
    
    var body: some View {
        VStack(spacing: 28) {
            HStack {
                VStack(spacing: 3) {
                    Rectangle().frame(height: 2)
                    Rectangle().frame(height: 2)
                }
                .frame(width: 16)
                
                Text("quick_menu")
                    .font(.semiBold(16))
                
                VStack(spacing: 3) {
                    Rectangle().frame(height: 2)
                    Rectangle().frame(height: 2)
                }
                .frame(width: 16)
            }
            .transformEffect(.init(a: 1, b: 0, c: -0.2, d: 1, tx: 0, ty: 0))
            .opacity(0.3)
            
            ScrollView(.horizontal) {
                LazyHStack {
                    ForEach(savestates) { savestate in
                        VStack {
                            if let image = CGImage.from(savestateData: savestate.image) {
                                Image(uiImage: UIImage(cgImage: image))
                                    .resizable()
                                    .aspectRatio(.screenWidth / .screenHeight, contentMode: .fit)
                                    .frame(width: .screenWidth)
                            }
                            
                            Text(savestate.timestamp.description)
                                .font(.semiBold(14))
                        }
                        .onTapGesture(perform: { sendInput(.didTapSavestate(savestate)) })
                    }
                }
                .padding(.horizontal, 32)
            }
            .frame(maxWidth: .infinity)
            .background(cardBackground)
            
            HStack {
                Spacer()
                action(name: "reset", input: .didTapReset, icon: "gobackward")
                Spacer()
                Rectangle()
                    .frame(width: 1)
                    .opacity(0.1)
                Spacer()
                action(name: "save", input: .didTapSave, icon: "square.and.arrow.down", offset: 4)
                Spacer()
                Rectangle()
                    .frame(width: 1)
                    .opacity(0.1)
                Spacer()
                action(name: "quit", input: .didTapExit, icon: "xmark.square")
                Spacer()
            }
            .frame(maxWidth: .infinity)
            .frame(height: 100)
            .padding()
            .background(cardBackground)
            
            Button(action: { sendInput(.didTapDismiss) }) {
                Image(systemName: "ellipsis.circle.fill")
                    .resizable()
                    .aspectRatio(1, contentMode: .fit)
                    .frame(width: 32, height: 32)
            }
        }
        .padding()
        .frame(maxWidth: .infinity)
        .background(.gameboy)
    }
}

#Preview {
    let config = ModelConfiguration(isStoredInMemoryOnly: true)
    let container = try! ModelContainer(for: Game.self, Savestate.self,
                                        configurations: config)
    
    let game = Game(name: "Allo", rom: .init(), importDate: .init())
    container.mainContext.insert(game)
    container.mainContext.insert(Savestate(timestamp: .now, game: game, image: .init(), data: .init()))
    
    return GameMenuView(sendInput: { _ in }, game: game)
        .modelContainer(container)
}
