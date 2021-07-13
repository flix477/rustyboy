//
//  View_Extension.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-13.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import SwiftUI

extension View {
    func cornerRadius(_ radius: CGFloat, corners: UIRectCorner) -> some View {
        clipShape(RoundedCorner(radius: radius, corners: corners))
    }
}
