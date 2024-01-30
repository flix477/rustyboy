//
//  StartSelectButton.swift
//  Rustyboy
//
//  Created by Félix Léveillé on 2024-01-11.
//

import SwiftUI

struct StartSelectButton: View {
    let name: String
    let didChangeHoldStatus: (Bool) -> Void
    
    @State
    private var isHolding = false
    
    var body: some View {
        VStack(spacing: 3) {
            ZStack {
                RoundedRectangle(cornerRadius: 16)
                    .fill(LinearGradient(colors: [.black, .black.opacity(0.1)],
                                         startPoint: .top,
                                         endPoint: .bottom))
                    .opacity(0.1)
                
                RoundedRectangle(cornerRadius: 16)
                    .fill(.black)
                    .padding(3)
                
                ZStack {
                    RoundedRectangle(cornerRadius: 16)
                        .fill(.gray)
                        .padding(4)
                    
                    RoundedRectangle(cornerRadius: 16)
                        .fill(LinearGradient(colors: [.white.opacity(0.4), .clear],
                                             startPoint: .top,
                                             endPoint: .bottom))
                        .padding(4)
                }
                .opacity(isHolding ? 0.9 : 1)
                .scaleEffect(.init(width: isHolding ? 0.95 : 1,
                                   height: isHolding ? 0.80 : 1))
            }
            .animation(.easeOut(duration: 0.1), value: isHolding)
            .aspectRatio(4, contentMode: .fit)
            .frame(width: 82)
            
            Text(name.uppercased())
                .font(.semiBold(16))
                .foregroundStyle(.logo)
                .scaleEffect(y: 0.8)
        }
        .gesture(DragGesture(minimumDistance: 0, coordinateSpace: .local)
            .onChanged { _ in isHolding = true }
            .onEnded { _ in isHolding = false })
        .rotationEffect(.radians(-.pi / 8))
        .onChange(of: isHolding) { didChangeHoldStatus($1) }
    }
}

#Preview {
    StartSelectButton(name: "Start", didChangeHoldStatus: { _ in })
}
