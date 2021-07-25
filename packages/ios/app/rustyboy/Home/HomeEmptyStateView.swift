//
//  HomeEmptyStateView.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-19.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import SwiftUI

struct HomeEmptyStateView: View {
    let addGamePressed: () -> Void

    var body: some View {
        VStack(spacing: 28) {
            Text("RUSTY BOY")
                .font(.semiBoldItalic(56))
                .foregroundColor(.init(red: 77, green: 78, blue: 141))

            Button(action: addGamePressed, label: {
                Text("Add game")
                    .font(.semiBold(24))
                    .foregroundColor(.white)
            })
            .cornerRadius(5)
            .padding(EdgeInsets(top: 24, leading: 64, bottom: 24, trailing: 64))
            .background(Color.tint)
        }
        .background(Color.white)
    }
}

struct HomeEmptyStateViewPreviews: PreviewProvider {
    static var previews: some View {
        HomeEmptyStateView(addGamePressed: {})
    }
}
