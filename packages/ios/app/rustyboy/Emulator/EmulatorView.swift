//
//  GameView.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-11.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import SwiftUI

struct EmulatorView<D>: View where D: HasPersistence & HasGameboy {
    let environment: D
    let quit: () -> Void

    @State private var showMenu: Bool = false

    private func onButtonEvent(button: ButtonType, state: ButtonEventType) {
        environment.gameboy.sendInput(buttonType: button, eventType: state)
    }

    var body: some View {
        VStack(spacing: 0) {
            ScreenView(gameboy: environment.gameboy)
                .aspectRatio(CGSize(width: CGFloat(SCREEN_WIDTH), height: CGFloat(SCREEN_HEIGHT)), contentMode: .fit)
                .cornerRadius(15, corners: .bottomRight)
                .cornerRadius(15, corners: .bottomLeft)

            HStack(alignment: .bottom, spacing: 3) {
                Text("Nintendo").font(.semiBold(10))
                Text("GAME BOY").font(.semiBoldItalic(12))
            }
            .foregroundColor(.white)
            .frame(minWidth: 0, maxWidth: .infinity, minHeight: 32, maxHeight: 32)
            .background(Color(red: 0, green: 0, blue: 0))

            GamepadView(onButtonEvent: onButtonEvent, onMenuPressed: { showMenu = true })
            .cornerRadius(15, corners: .bottomLeft)
            .cornerRadius(15, corners: .bottomRight)
        }
        .shadow(radius: 10)
        .onDisappear {
            EmulatorController.dumpSavestate().unsafeRunAsync(with: environment, on: .main)
        }
        .sheet(isPresented: $showMenu) {
            EmulatorMenuView(environment: environment, dismiss: { showMenu = false }, quit: quit)
        }
    }
}
