//
//  EmulatorMenuActionsView.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-20.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import SwiftUI

struct EmulatorMenuActionsView: View {
    let onReset: () -> Void
    let onSave: () -> Void
    let onQuit: () -> Void

    var body: some View {
        VStack(spacing: 16) {
            EmulatorMenuButton(title: "Save", image: "save_white", action: onSave)

            HStack(spacing: 16) {
                EmulatorMenuButton(title: "Reset", image: "reset_white", action: onReset)
                EmulatorMenuButton(title: "Quit", image: "close_white", action: onQuit)
            }
        }
        .frame(maxWidth: 256)
    }
}

struct EmulatorMenuActionsViewPreviews: PreviewProvider {
    static var previews: some View {
        EmulatorMenuActionsView(onReset: {}, onSave: {}, onQuit: {})
    }
}
