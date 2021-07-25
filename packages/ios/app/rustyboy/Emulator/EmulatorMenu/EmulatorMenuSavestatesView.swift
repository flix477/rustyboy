//
//  EmulatorMenuSavestatesView.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-20.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import SwiftUI

struct EmulatorMenuSavestatesView: View {
    let savestates: [SavestateViewModel]
    let loading: Bool
    let onSavestateTap: (String) -> () -> Void

    let rows = [
        GridItem()
    ]

    var body: some View {
        if loading {
            ProgressView()
        } else {
            ScrollView(.horizontal, showsIndicators: false) {
                LazyHGrid(rows: rows, spacing: 16) {
                    ForEach(savestates, id: \.id) { savestate in
                        Image(uiImage: savestate.image)
                            .resizable()
                            .aspectRatio(contentMode: .fit)
                            .frame(width: 160,
                                   height: 160 * CGFloat(SCREEN_HEIGHT) / CGFloat(SCREEN_WIDTH))
                            .onTapGesture(perform: onSavestateTap(savestate.id))
                    }
                }
                .padding(EdgeInsets(top: 0, leading: 16, bottom: 0, trailing: 16))
            }
        }
    }
}

struct EmulatorMenuSavestatesViewPreviews: PreviewProvider {
    static var previews: some View {
        EmulatorMenuSavestatesView(savestates: [SavestateViewModel(id: "1",
                                                                   createdAt: Date(),
                                                                   image: UIImage(named: "savestate_preview")!),
                                                SavestateViewModel(id: "2",
                                                                   createdAt: Date(),
                                                                   image: UIImage(named: "savestate_preview")!),
                                                SavestateViewModel(id: "3",
                                                                   createdAt: Date(),
                                                                   image: UIImage(named: "savestate_preview")!),
                                                SavestateViewModel(id: "4",
                                                                   createdAt: Date(),
                                                                   image: UIImage(named: "savestate_preview")!)],
                                   loading: false,
                                   onSavestateTap: { _ in {} })
    }
}
