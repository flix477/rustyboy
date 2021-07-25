//
//  DocumentPicker.swift
//  rustyboy
//
//  Created by Felix Leveille on 2021-07-11.
//  Copyright © 2021 Félix Léveillé. All rights reserved.
//

import Foundation
import SwiftUI
import UIKit
import UniformTypeIdentifiers

extension UTType {
    static let rom = UTType("com.fleveille.rustyboy.gb")!
}

struct DocumentPicker: UIViewControllerRepresentable {
    typealias UIViewControllerType = UIDocumentPickerViewController

    let didSelectURLs: ([URL]) -> Void

    func makeUIViewController(context: Context) -> UIDocumentPickerViewController {
        let vc = UIDocumentPickerViewController(forOpeningContentTypes: [.data], asCopy: true)
        vc.delegate = context.coordinator

        return vc
    }

    func updateUIViewController(_ uiViewController: UIDocumentPickerViewController, context: Context) {}

    func makeCoordinator() -> DocumentPickerCoordinator {
        return DocumentPickerCoordinator(didSelectURLs: didSelectURLs)
    }
}

class DocumentPickerCoordinator: NSObject, UIDocumentPickerDelegate {
    private let didSelectURLs: ([URL]) -> Void

    init(didSelectURLs: @escaping ([URL]) -> Void) {
        self.didSelectURLs = didSelectURLs
    }

    func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
        self.didSelectURLs(urls)
    }
}
