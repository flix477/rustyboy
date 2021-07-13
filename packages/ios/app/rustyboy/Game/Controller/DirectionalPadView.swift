import Foundation
import SwiftUI

struct DirectionalPadView: View {
    static let margin = CGFloat(8)
    static let buttonSize = CGFloat(56)

    var onButtonEvent: ((ButtonType, ButtonEventType) -> Void)?

    @State private var pressed: ButtonType?

    private func button(at position: CGPoint) -> ButtonType {
        let (x, y) = (position.x, position.y)

        if x < 0 && y > x && y < -x { return .left }
        if x > 0 && y < x && y > -x { return .right }
        return y < 0 ? .up : .down
    }

    private func drag(withGeometry geometry: GeometryProxy) -> some Gesture {
        let frame = geometry.frame(in: .local)
        let middle = CGPoint(x: frame.width / 2, y: frame.height / 2)

        return DragGesture(minimumDistance: 0, coordinateSpace: .local)
            .onChanged { value in
                let position = CGPoint(x: value.location.x - middle.x, y: value.location.y - middle.y)
                let button = button(at: position)

                if self.pressed == button { return }

                if let pressed = self.pressed {
                    self.onButtonEvent?(pressed, .up)
                }

                self.pressed = button
                UIImpactFeedbackGenerator().impactOccurred()
                self.onButtonEvent?(button, .down)
            }
            .onEnded { _ in
                if let pressed = self.pressed {
                    self.onButtonEvent?(pressed, .up)
                }
                self.pressed = nil
            }
    }

    var body: some View {
        ZStack {
            GeometryReader { geometry in
                Circle()
                    .fill(self.pressed == nil ? Color.buttonBackground : Color.buttonPressedBackground)
                    .gesture(drag(withGeometry: geometry))
            }
            Image("dpad")
                .resizable()
                .aspectRatio(contentMode: .fit)
                .allowsHitTesting(false)
        }
    }
}

struct DirectionalPadPreviews: PreviewProvider {
    static var previews: some View {
        DirectionalPadView()
    }
}
