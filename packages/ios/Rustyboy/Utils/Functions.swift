import OrderedCollections
import Slingshot

public func groupc<ID, ConditionValue, GroupsSequence, ElementSequence>(
    orderedElements elements: ElementSequence,
    inOrderedGroups groups: GroupsSequence,
    byEvaluating isInGroup: @escaping (ElementSequence.Element, ConditionValue) -> Bool
) -> OrderedDictionary<ID, [ElementSequence.Element]>
where ElementSequence: Sequence, GroupsSequence: Sequence, GroupsSequence.Element == (id: ID, condition: ConditionValue) {
    fold2(initial: [:],
          left: elements,
          right: groups) { output, element, group in
        guard isInGroup(element, group.condition) else {
            return .consumeRight
        }
        
        output.updateValue(forKey: group.id,
                           default: [],
                           with: { $0.append(element) })
        return .consumeLeft
    }
}
