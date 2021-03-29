# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @return {Integer[]}
def next_larger_nodes(head)
  curr = head
  vals = []

  until curr.nil?
    vals.push(curr.val)
    curr = curr.next
  end

  stack = []
  ret = [0] * vals.size

  (vals.size - 1..0).step(-1).each do |i|
    stack.pop until stack.empty? || vals[i] < stack[-1]
    ret[i] = stack[-1] unless stack.empty?
    stack.push(vals[i])
  end

  ret
end
