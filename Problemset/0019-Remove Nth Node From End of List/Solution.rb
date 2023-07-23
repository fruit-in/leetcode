# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @param {Integer} n
# @return {ListNode}
def remove_nth_from_end(head, n)
    dummy = ListNode.new(0, head)
    node0 = head
    node1 = dummy

    for _ in 0...n
        node0 = node0.next
    end

    while node0
        node0 = node0.next
        node1 = node1.next
    end

    node1.next = node1.next.next

    return dummy.next
end
