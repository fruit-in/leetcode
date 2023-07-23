# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @return {ListNode}
def delete_duplicates(head)
  dummy = ListNode.new(0, head)
  curr = dummy

  until curr.nil?
    if !curr.next.nil? && !curr.next.next.nil? && curr.next.val == curr.next.next.val
      curr.next.next = curr.next.next.next while !curr.next.next.nil? && curr.next.val == curr.next.next.val
      curr.next = curr.next.next
    else
      curr = curr.next
    end
  end

  dummy.next
end
