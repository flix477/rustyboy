import Foundation

extension Sequence {
    func prefix<Head>(by head: Head) -> PrefixSequence<Head, Self>
    where Head: Sequence, Head.Element == Element {
        .init(prefix: head, tail: self)
    }
}
