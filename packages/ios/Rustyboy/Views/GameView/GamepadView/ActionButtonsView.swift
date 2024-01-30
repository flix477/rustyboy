//
//  ActionButtonsView.swift
//  Rustyboy
//
//  Created by Félix Léveillé on 2024-01-11.
//

import SwiftUI

struct ActionButtonsView: View {
    let didChangeHeldActionButtons: (Set<ActionButton>) -> Void
    
    private let buttonWidthMultiplier = 0.4
    
    enum ActionButton {
        case a, b
    }
    
    @State
    private var heldButtons: Set<ActionButton> = []
    
    @State
    private var firstHeldButton: ActionButton?
    
    private func button(name: String, action: ActionButton, geom: GeometryProxy) -> some View {
        VStack(spacing: 16) {
            let held = heldButtons.contains(action)
            ZStack {
                Circle()
                    .fill(.black)
                
                ZStack {
                    Circle()
                        .fill(.accent)
                        .shadow(radius: 4)
                    
                    Circle()
                        .fill(LinearGradient(colors: [.white, .black],
                                             startPoint: .top,
                                             endPoint: .bottom))
                        .opacity(0.15)
                    
                    Circle()
                        .fill(.accent)
                        .padding(geom.size.width * buttonWidthMultiplier * 0.07)
                }
                .opacity(held ? 0.9 : 1)
                .scaleEffect(.init(width: held ? 0.85 : 1,
                                   height: held ? 0.85 : 1))
            }
            .animation(.easeOut(duration: 0.1), value: held)
            .aspectRatio(1, contentMode: .fit)
            .frame(width: geom.size.width * buttonWidthMultiplier)
            
            Text(name)
                .font(.semiBold(22))
                .foregroundStyle(.logo)
                .scaleEffect(y: 0.8)
        }
    }
    
    private func button(at position: CGPoint, in geometry: GeometryProxy) -> ActionButton? {
        let buttonWidth = geometry.size.width * buttonWidthMultiplier
        
        guard position.y < buttonWidth else { return nil }
        
        if position.x < buttonWidth {
            return .b
        } else if position.x > buttonWidth + 16 {
            return .a
        }
        
        return nil
    }
    
    var body: some View {
        GeometryReader { geom in
            ZStack {
                RoundedRectangle(cornerRadius: (geom.size.width * buttonWidthMultiplier + 16) / 2)
                    .fill(LinearGradient(colors: [.black, .black.opacity(0.3)],
                                         startPoint: .top,
                                         endPoint: .bottom))
                    .frame(width: geom.size.width * buttonWidthMultiplier * 2 + 16 + 16,
                           height: geom.size.width * buttonWidthMultiplier + 16)
                    .offset(y: -21)
                    .opacity(0.1)
                
                HStack(spacing: 16) {
                    button(name: "B", action: .b, geom: geom)
                    button(name: "A", action: .a, geom: geom)
                }
                .gesture(DragGesture(minimumDistance: 0, coordinateSpace: .local)
                    .onChanged { value in
                        let button = button(at: value.location, in: geom)
                        if let button, firstHeldButton == nil {
                            firstHeldButton = button
                            heldButtons.insert(button)
                        } else if let firstHeldButton, button == firstHeldButton || button == nil {
                            heldButtons = [firstHeldButton]
                        } else if let firstHeldButton, let button, button != firstHeldButton {
                            heldButtons.insert(button)
                        }
                    }
                    .onEnded { _ in
                        firstHeldButton = nil
                        heldButtons = []
                    })
            }
            .rotationEffect(.degrees(-20))
            .onChange(of: heldButtons) { didChangeHeldActionButtons($1) }
        }
    }
}

#Preview {
    ActionButtonsView(didChangeHeldActionButtons: { _ in })
}
