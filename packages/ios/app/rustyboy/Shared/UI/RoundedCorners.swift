//
//  RoundedCorners.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-13.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import SwiftUI

struct RoundedCorner: Shape {
    var radius: CGFloat = .infinity
    var corners: UIRectCorner = .allCorners

    func path(in rect: CGRect) -> Path {
        let path = UIBezierPath(roundedRect: rect,
                                byRoundingCorners: corners,
                                cornerRadii: CGSize(width: radius, height: radius))
        return Path(path.cgPath)
    }
}
