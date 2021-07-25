//
//  EmulatorMenuView.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-19.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import SwiftUI
import BowEffects

struct EmulatorMenuView<D: HasGameboy & HasPersistence>: View {
    let environment: D
    let dismiss: () -> Void
    let quit: () -> Void
    @State private var loading: Bool = true
    @State private var savestates: [(Savestate, SavestateViewModel)] = []

    private func fetchSavestates() {
        self.loading = true
        EmulatorMenuController.savestates()
            .andThen { xs in
                xs.map { x in SavestateViewModel.from(savestate: x)(environment.game).map { (x, $0) } }.sequence()
            }
            .unsafeRunAsync(with: environment, on: .main) { result in
            switch result.toResult() {
            case .success(let savestates):
                self.savestates = savestates
            case .failure(let error):
                print(error)
            }

            self.loading = false
        }
    }

    private func onSavestateTap(savestate id: String) -> () -> Void {
        let savestate: Savestate = savestates.first { $0.1.id == id }!.0
        return {
            EmulatorMenuController.loadSavestate(savestate: savestate)
                .unsafeRunAsync(with: environment, on: .main) { result in
                if case .failure(let error) = result.toResult() {
                    print(error)
                } else {
                    dismiss()
                }
            }
        }
    }

    private func onReset() {
        EmulatorMenuController.reset().unsafeRunAsync(with: environment, on: .main) { result in
            if case .failure(let error) = result.toResult() {
                print(error)
            } else {
                dismiss()
            }
        }
    }

    private func onSave() {
        EmulatorMenuController.dumpSavestate().unsafeRunAsync(with: environment, on: .main) { result in
            if case .failure(let error) = result.toResult() {
                print(error)
            } else {
                fetchSavestates()
                dismiss()
            }
        }
    }

    var body: some View {
        EmulatorMenuDumbView(savestates: savestates.map(\.1),
                             onSavestateTap: onSavestateTap,
                             loading: loading,
                             onReset: onReset,
                             onSave: onSave,
                             onQuit: quit)
            .onAppear(perform: fetchSavestates)
    }
}

struct EmulatorMenuDumbView: View {
    let savestates: [SavestateViewModel]
    let onSavestateTap: (String) -> () -> Void
    let loading: Bool
    let onReset: () -> Void
    let onSave: () -> Void
    let onQuit: () -> Void

    var body: some View {
        VStack(spacing: 16) {
            HStack {
                Rectangle()
                    .opacity(0.3)
                    .frame(width: 24, height: 1)

                Text("QUICK MENU")
                    .kerning(0.7)
                    .font(.semiBold(12))
                    .opacity(0.3)

                Rectangle()
                    .opacity(0.3)
                    .frame(width: 24, height: 1)
            }

            EmulatorMenuSavestatesView(savestates: savestates,
                                       loading: loading,
                                       onSavestateTap: onSavestateTap)

            EmulatorMenuActionsView(onReset: onReset,
                                    onSave: onSave,
                                    onQuit: onQuit)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .padding(EdgeInsets(top: 32, leading: 0, bottom: 32, trailing: 0))
        .background(LinearGradient(gradient: .primary,
                                   startPoint: .topLeading,
                                   endPoint: .bottomTrailing).ignoresSafeArea())
        .foregroundColor(.white)
    }
}

struct EmulatorMenuDumbViewPreviews: PreviewProvider {
    static var previews: some View {
        EmulatorMenuDumbView(savestates: [SavestateViewModel(id: "1",
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
                             onSavestateTap: { _ in {} },
                             loading: false,
                             onReset: {},
                             onSave: {},
                             onQuit: {})
    }
}
