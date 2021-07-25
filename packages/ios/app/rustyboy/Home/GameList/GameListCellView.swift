//
//  GameListRow.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-18.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import SwiftUI

struct GameListCellView: View {
    let model: GameViewModel
    @State private var pressed = false

    var body: some View {
        VStack(spacing: 12) {
            Image("cartridge")
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(width: 160, height: 160)
                .shadow(radius: 10)

            Text(model.name)
                .font(.semiBold(16))
        }
        .contextMenu(ContextMenu(menuItems: {
            Text("Play")
            Text("Delete")
        }))
    }
}

struct GameListCellViewPreviews: PreviewProvider {
    static var previews: some View {
        GameListCellView(model: GameViewModel(id: "1", name: "Test"))
    }
}
