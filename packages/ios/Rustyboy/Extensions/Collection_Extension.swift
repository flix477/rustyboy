import Foundation

extension Sequence {
    func max<T>(by keyPath: KeyPath<Element, T>) -> Element? where T: Comparable {
        self.max(by: { $0[keyPath: keyPath] < $1[keyPath: keyPath] })
    }
}
