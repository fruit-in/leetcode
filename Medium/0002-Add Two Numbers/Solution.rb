# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} l1
# @param {ListNode} l2
# @return {ListNode}
def add_two_numbers(l1, l2)
    l3 = ListNode.new
    curr = l3
    carry = 0

    while l1 or l2
        curr.next = ListNode.new(carry)
        curr = curr.next
        curr.val += l1.val if l1
        curr.val += l2.val if l2
        carry = curr.val / 10
        curr.val %= 10

        l1 = l1.next if l1
        l2 = l2.next if l2
    end

    curr.next = ListNode.new(carry) if carry == 1

    return l3.next
end
