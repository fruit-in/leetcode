# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} list1
# @param {Integer} a
# @param {Integer} b
# @param {ListNode} list2
# @return {ListNode}
def merge_in_between(list1, a, b, list2)
  removed_a = list1
  (1...a).each do |_|
    removed_a = removed_a.next
  end
  removed_b = removed_a.next
  (0..(b - a)).each do |_|
    removed_b = removed_b.next
  end

  curr = list2
  curr = curr.next until curr.next.nil?

  removed_a.next = list2
  curr.next = removed_b

  list1
end
