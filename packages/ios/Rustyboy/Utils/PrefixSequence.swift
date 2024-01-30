import Foundation

public struct PrefixSequence<Prefix, Tail>: Sequence
where Prefix: Sequence, Tail: Sequence, Prefix.Element == Tail.Element {
    let prefix: Prefix
    let tail: Tail
    
    public struct PrefixIterator: IteratorProtocol {
        private var prefix: Prefix.Iterator
        private var tail: Tail.Iterator
        
        init(prefix: Prefix, tail: Tail) {
            self.prefix = prefix.makeIterator()
            self.tail = tail.makeIterator()
        }
        
        public mutating func next() -> Tail.Element? {
            prefix.next() ?? tail.next()
        }
    }
    
    public func makeIterator() -> PrefixIterator {
        .init(prefix: prefix, tail: tail)
    }
}
