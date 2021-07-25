//
//  ScreenView.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-12.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import MetalKit
import SwiftUI

struct ScreenView: UIViewRepresentable {
    typealias UIViewType = MTKView
    let gameboy: Gameboy
    private let device = MTLCreateSystemDefaultDevice()!

    func makeCoordinator() -> MTKViewDelegate {
        return Renderer(device: device, onDraw: {
            self.gameboy.runToVblank()
        })!
    }

    func makeUIView(context: Context) -> MTKView {
        let mtkView = MTKView()
        mtkView.device = device
        mtkView.colorPixelFormat = .bgra8Unorm
        mtkView.isOpaque = true
        mtkView.preferredFramesPerSecond = 60
        mtkView.delegate = context.coordinator
        mtkView.backgroundColor = .blue
        mtkView.autoResizeDrawable = true
        mtkView.drawableSize = mtkView.frame.size

        return mtkView
    }

    func updateUIView(_ uiView: MTKView, context: Context) {}
}
