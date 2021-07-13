import Foundation
import SwiftUI

private struct GameButtonModifier: ViewModifier {
    let action: (ButtonEventType) -> Void
    @State private var pressed = false

    func body(content: Content) -> some View {
        return content.gesture(DragGesture(minimumDistance: 0)
            .onChanged { _ in
                if pressed { return }
                UIImpactFeedbackGenerator().impactOccurred()
                self.action(.down)
                pressed = true
            }
            .onEnded { _ in
                if !pressed { return }
                self.action(.up)
                pressed = false
            })
    }
}

struct ControllerView: View {
    var onButtonEvent: ((ButtonType, ButtonEventType) -> Void)?

    func buttonAction(for buttonType: ButtonType) -> (ButtonEventType) -> Void {
        return { state in self.onButtonEvent?(buttonType, state) }
    }

    var body: some View {
        VStack(spacing: 32) {
            HStack {
                DirectionalPadView(onButtonEvent: onButtonEvent)
                    .frame(width: 168, height: 168, alignment: .center)
                Spacer()
                HStack {
                    Circle()
                        .fill(Color.buttonBackground)
                        .frame(width: 84, height: 84)
                        .padding(EdgeInsets(top: 11, leading: 0, bottom: 0, trailing: 0))
                        .modifier(GameButtonModifier(action: buttonAction(for: .b)))

                    Circle()
                        .fill(Color.buttonBackground)
                        .frame(width: 84, height: 84)
                        .padding(EdgeInsets(top: 0, leading: 0, bottom: 54, trailing: 0))
                        .modifier(GameButtonModifier(action: buttonAction(for: .a)))
                }
                .frame(height: 150)
                .background(Image("abbuttons").resizable().aspectRatio(contentMode: .fit))
            }

            HStack(spacing: 0) {
                Image("select")
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(width: 64, height: 42)
                    .modifier(GameButtonModifier(action: buttonAction(for: .select)))

                Image("start")
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(width: 64, height: 42)
                    .modifier(GameButtonModifier(action: buttonAction(for: .start)))
            }
        }
        .padding(EdgeInsets(top: 32, leading: 16, bottom: 32, trailing: 16))
        .background(Color("Controller"))
    }
}

struct ControllerPreviews: PreviewProvider {
    static var previews: some View {
        ControllerView()
    }
}
