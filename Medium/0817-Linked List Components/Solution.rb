# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @param {Integer[]} g
# @return {Integer}
def num_components(head, g)
  g = g.to_set
  curr = head
  ret = 0

  until curr.nil?
    ret += 1 if g.member?(curr.val) && (curr.next.nil? || !g.member?(curr.next.val))

    curr = curr.next
  end

  ret
end
