import SwiftUI

struct Cross: Shape {
    let percentage: CGFloat
    
    func path(in rect: CGRect) -> Path {
        let horizontal = (rect.width - rect.width * percentage) / 2
        let vertical = (rect.height - rect.height * percentage) / 2
        
        var path = Path()
        path.move(to: .init(x: horizontal, y: 0))
        path.addLine(to: .init(x: rect.width - horizontal, y: 0))
        path.addLine(to: .init(x: rect.width - horizontal, y: vertical))
        path.addLine(to: .init(x: rect.width, y: vertical))
        path.addLine(to: .init(x: rect.width, y: rect.height - vertical))
        path.addLine(to: .init(x: rect.width - horizontal, y: rect.height - vertical))
        path.addLine(to: .init(x: rect.width - horizontal, y: rect.height))
        path.addLine(to: .init(x: horizontal, y: rect.height))
        path.addLine(to: .init(x: horizontal, y: rect.height - vertical))
        path.addLine(to: .init(x: 0, y: rect.height - vertical))
        path.addLine(to: .init(x: 0, y: vertical))
        path.addLine(to: .init(x: horizontal, y: vertical))
        path.closeSubpath()
        
        return path
    }
}

struct DirectionalPadView: View {
    let didChangeDirection: (Direction?) -> Void
    
    private static let maxTiltAngle = Angle.degrees(10)
    
    @State
    private var position: CGPoint? = nil
    
    private var angle: Angle? {
        guard let position else { return nil }
        
        var angle = atan((abs(position.y - 0.5)) / (abs(position.x - 0.5)))
        
        if position.x < 0.5 && position.y < 0.5 {
            angle = (.pi - angle)
        } else if position.x < 0.5 {
            angle += .pi
        } else if position.y >= 0.5 {
            angle = (2 * .pi - angle)
        }
        
        return Angle(radians: angle)
    }
    
    private var direction: Direction? {
        angle.map { angle in
            Direction.allCases
                .first(where: { angle < $0.maxAngle }) ?? .right
        }
    }
    
    private var sectorProgression: CGFloat? {
        guard let direction, let angle else { return nil }
        
        var minAngle = direction.minAngle
        var maxAngle = direction.maxAngle
        if minAngle > maxAngle {
            if angle.radians < .pi {
                minAngle = -maxAngle
            } else {
                maxAngle = .radians(2 * .pi + maxAngle.radians)
            }
        }
        
        return (angle.radians - minAngle.radians) / (maxAngle.radians - minAngle.radians)
    }
    
    enum Direction: Int, CaseIterable {
        case right = 0
        case up = 1
        case left = 2
        case down = 3
        
        var maxAngle: Angle {
            Angle.radians(.pi / 4 + Double(rawValue) * 2 * .pi / 4)
        }
        
        var previous: Self {
            .init(rawValue: rawValue - 1) ?? .down
        }
        
        var minAngle: Angle {
            previous.maxAngle
        }
        
        var unitPoints: (UnitPoint, UnitPoint) {
            switch self {
            case .right:
                (.leading, .trailing)
            case .up:
                (.bottom, .top)
            case .left:
                (.trailing, .leading)
            case .down:
                (.top, .bottom)
            }
        }
    }
    
    private var rotationAxis: (x: CGFloat, y: CGFloat, z: CGFloat) {
        switch direction {
        case .right, .left:
            return (0, 1, 0)
        case .up, .down:
            return (1, 0, 0)
        case nil:
            return (0, 0, 0)
        }
    }
    
    private var tiltAngle: Angle {
        switch direction {
        case .up, .right:
            Self.maxTiltAngle
        case .down, .left:
            -Self.maxTiltAngle
        case nil:
            .zero
        }
    }
    
    private func grip(geom: GeometryProxy) -> some View {
        ZStack {
            RoundedRectangle(cornerRadius: 10)
                .fill(LinearGradient(colors: [.white, .black.opacity(0.5)],
                                     startPoint: .top,
                                     endPoint: .bottom))
            
            RoundedRectangle(cornerRadius: 10)
                .padding(geom.size.width * 0.006)
        }
        .frame(width: geom.size.width * 0.23,
               height: geom.size.height * 0.08)
        .opacity(0.1)
    }
    
    private func hGrip(geom: GeometryProxy) -> some View {
        ZStack {
            RoundedRectangle(cornerRadius: 10)
                .fill(LinearGradient(colors: [.white, .black.opacity(0.5)],
                                     startPoint: .top,
                                     endPoint: .bottom))
            
            RoundedRectangle(cornerRadius: 10)
                .padding(geom.size.width * 0.006)
        }
        .frame(width: geom.size.height * 0.08,
               height: geom.size.width * 0.23)
        .opacity(0.1)
    }
    
    var body: some View {
        GeometryReader { geom in
            ZStack {
                Cross(percentage: 0.34)
                    .fill(Color.black)
                
                ZStack {
                    Cross(percentage: 0.34)
                        .fill(Color.dpad)
                        .shadow(radius: 2)
                    
                    Cross(percentage: 0.30)
                        .fill(Color.white)
                        .padding(geom.size.width * 0.02)
                        .opacity(0.2)
                    
                    VStack(spacing: geom.size.width * 0.02) {
                        grip(geom: geom)
                        grip(geom: geom)
                        grip(geom: geom)
                    }
                    .offset(y: geom.size.width * 0.32)
                    
                    VStack(spacing: geom.size.width * 0.02) {
                        grip(geom: geom)
                        grip(geom: geom)
                        grip(geom: geom)
                    }
                    .offset(y: geom.size.width * -0.32)
                    
                    HStack(spacing: geom.size.width * 0.02) {
                        hGrip(geom: geom)
                        hGrip(geom: geom)
                        hGrip(geom: geom)
                    }
                    .offset(x: geom.size.width * -0.32)
                    
                    HStack(spacing: geom.size.width * 0.02) {
                        hGrip(geom: geom)
                        hGrip(geom: geom)
                        hGrip(geom: geom)
                    }
                    .offset(x: geom.size.width * 0.32)
                    
                    ZStack {
                        Circle()
                            .fill(RadialGradient(colors: [.black, .black.opacity(0.2)],
                                                 center: .center,
                                                 startRadius: .zero,
                                                 endRadius: geom.size.width * 0.15))
                        
                        Circle()
                            .padding(geom.size.width * 0.03)
                            .opacity(0.1)
                    }
                    .opacity(0.3)
                    .frame(width: geom.size.width * 0.30,
                           height: geom.size.width * 0.30)
                }
                .rotation3DEffect(tiltAngle, axis: rotationAxis)
//                .mask {
//                    LinearGradient(colors: [.black, position == nil ? .black : .black.opacity(0.8)],
//                                   startPoint: direction?.unitPoints.0 ?? .top,
//                                   endPoint: direction?.unitPoints.1 ?? .bottom)
//                }
            }
            .animation(.easeOut(duration: 0.1), value: rotationAxis.x)
            .animation(.easeOut(duration: 0.1), value: rotationAxis.y)
            .animation(.easeOut(duration: 0.1), value: tiltAngle)
            .gesture(DragGesture(minimumDistance: .zero,
                                 coordinateSpace: .local)
                .onChanged { value in
                    position = CGPoint(x: value.location.x / geom.size.width,
                                       y: value.location.y / geom.size.height)
                }
                .onEnded { _ in
                    position = nil
                })
        }
        .onChange(of: direction) { _, new in
            didChangeDirection(new)
        }
        .aspectRatio(1, contentMode: .fit)
    }
}

#Preview {
    DirectionalPadView(didChangeDirection: { print($0 as Any) })
}
