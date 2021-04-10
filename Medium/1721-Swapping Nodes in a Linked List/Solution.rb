# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @param {Integer} k
# @return {ListNode}
def swap_nodes(head, k)
  dummy = ListNode.new(0, head)
  curr = head
  n = 0
  until curr.nil?
    n += 1
    curr = curr.next
  end

  curr = dummy
  node0 = nil
  node1 = nil
  (0..n).each do |i|
    node0 = curr if i == k - 1
    node1 = curr if i == n - k
    curr = curr.next
  end

  temp = node0.next
  node0.next = node1.next
  node1.next = temp
  temp = node0.next.next
  node0.next.next = node1.next.next
  node1.next.next = temp

  dummy.next
end
