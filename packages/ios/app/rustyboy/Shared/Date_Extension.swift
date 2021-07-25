//
//  Date_Extension.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-19.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import Bow
import BowEffects

extension Date {
    static func now() -> UIO<Date> {
        return UIO.invoke { Date() }
    }
}
