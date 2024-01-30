import SwiftUI
import Slingshot

struct GamepadView: View {
    let didChangeDirection: (DirectionalPadView.Direction?) -> Void
    let didChangeHeldButtons: (ButtonSet) -> Void
    let didPressMenuButton: () -> Void
    
    struct ButtonSet: OptionSet {
        let rawValue: UInt8
        
        static let a = ButtonSet(rawValue: 1)
        static let b = ButtonSet(rawValue: 2)
        static let select = ButtonSet(rawValue: 4)
        static let start = ButtonSet(rawValue: 8)
    }
    
    @State
    private var heldButtons: ButtonSet = []
    
    init(didChangeDirection: @escaping (DirectionalPadView.Direction?) -> Void, 
         didChangeHeldButtons: @escaping (ButtonSet) -> Void,
         didPressMenuButton: @escaping () -> Void) {
        self.didChangeDirection = didChangeDirection
        self.didChangeHeldButtons = didChangeHeldButtons
        self.didPressMenuButton = didPressMenuButton
    }
    
    private func didChange(heldActionButtons: Set<ActionButtonsView.ActionButton>) {
        if heldActionButtons.contains(.a) {
            heldButtons.insert(.a)
        } else {
            heldButtons.remove(.a)
        }
        
        if heldActionButtons.contains(.b) {
            heldButtons.insert(.b)
        } else {
            heldButtons.remove(.b)
        }
    }
    
    var body: some View {
        GeometryReader { geom in
            VStack(spacing: 32) {
                HStack(alignment: .center, spacing: 0) {
                    DirectionalPadView(didChangeDirection: didChangeDirection)
                        .frame(width: geom.size.width / 2)
                    
                    Spacer()
                    
                    ActionButtonsView(didChangeHeldActionButtons: didChange(heldActionButtons:))
                        .frame(width: geom.size.width / 2.5, height: geom.size.width / 3)
                        .offset(y: 16)
                }
                
                HStack(spacing: 16) {
                    StartSelectButton(name: "Select",
                                      didChangeHoldStatus: {
                        if $0 {
                            heldButtons.insert(.select)
                        } else {
                            heldButtons.remove(.select)
                        }
                    })
                    
                    StartSelectButton(name: "Start",
                                      didChangeHoldStatus: {
                        if $0 {
                            heldButtons.insert(.start)
                        } else {
                            heldButtons.remove(.start)
                        }
                    })
                }
                
                Spacer()
                
                Button(action: didPressMenuButton) {
                    Image(systemName: "ellipsis.circle.fill")
                        .resizable()
                        .aspectRatio(1, contentMode: .fit)
                        .frame(width: 32, height: 32)
                }
                .padding(.bottom)
            }
            .onChange(of: heldButtons) {
                didChangeHeldButtons($1)
            }
        }
    }
}

#Preview {
    GamepadView(didChangeDirection: { _ in }, 
                didChangeHeldButtons: { _ in },
                didPressMenuButton: {})
}
