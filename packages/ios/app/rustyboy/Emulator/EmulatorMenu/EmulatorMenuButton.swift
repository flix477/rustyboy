//
//  EmulatorMenuButton.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-20.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import SwiftUI

struct EmulatorMenuButton: View {
    let title: String
    let image: String
    let action: () -> Void

    var body: some View {
        Button(action: action, label: {
            HStack {
                Image(image)
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(width: 25, height: 25)
                Text(title)
                    .font(.semiBold(16))
                    .opacity(0.8)
            }
        })
        .frame(maxWidth: .infinity)
        .padding(EdgeInsets(top: 16, leading: 24, bottom: 16, trailing: 24))
        .background(Color(red: 255, green: 255, blue: 255, opacity: 0.2))
        .cornerRadius(6)
    }
}
