import SwiftUI
import SwiftData
import RustyboyCoreBindings

struct GameView: View {
    @Environment(\.modelContext)
    private var modelContext
    
    @Environment(\.scenePhase)
    private var scenePhase
    
    @State
    private var gameboy: RustyboyGameboy?
    
    @State
    private var viewModel: GameViewModel
    
    private var dismiss: () -> Void
    
    init(viewModel: GameViewModel,
         dismiss: @escaping () -> Void) {
        self.dismiss = dismiss
        self._viewModel = .init(initialValue: viewModel)
    }
    
    private func didPressMenuButton() {
        guard let gameboy else { return }
        
        withAnimation {
            viewModel.pause(withGameboy: gameboy)
        }
    }
    
    private func on(gameMenuInput input: GameMenuView.Input) {
        guard let gameboy else { return }
        
        switch input {
        case .didTapSavestate(let savestate):
            let result = viewModel.load(savestate: savestate, withGameboy: gameboy)
            if let failure = result.failure {
                print(failure)
            }
        case .didTapReset:
            viewModel.reset(gameboy: gameboy)
        case .didTapSave:
            if let savestate = viewModel.save(gameboy: gameboy) {
                modelContext.insert(savestate)
            }
        case .didTapDismiss:
            withAnimation {
                viewModel.resume()
            }
        case .didTapExit:
            dismiss()
        }
    }
    
    var body: some View {
        Group {
            if let pauseScreen = viewModel.pauseScreen {
                GameMenuView(sendInput: on(gameMenuInput:), game: viewModel.game)
                    .transition(.move(edge: .bottom))
            } else {
                VStack(spacing: 16) {
                    if let gameboy {
                        ScreenView(render: viewModel.renderer(withGameboy: gameboy))
                            .aspectRatio(.screenWidth / .screenHeight, contentMode: .fit)
                    }
                    
                    Spacer()
                    
                    GamepadView(didChangeDirection: viewModel.didChange(direction:),
                                didChangeHeldButtons: viewModel.didChange(heldButtons:),
                                didPressMenuButton: didPressMenuButton)
                }
                .background {
                    ZStack {
                        Color.gameboy
                        
                        LinearGradient(colors: [.clear, .black.opacity(0.05)],
                                       startPoint: .top,
                                       endPoint: .bottom)
                    }
                    .ignoresSafeArea()
                }
            }
        }
        .onAppear {
            switch viewModel.start() {
            case .success(let gameboy):
                self.gameboy = gameboy
            case .failure(let error):
                print(error)
            }
        }
        .onChange(of: scenePhase) { oldPhase, newPhase in
            if newPhase == .background {
                if let gameboy {
                    viewModel.pause(withGameboy: gameboy)
                }
            }
        }
    }
}
