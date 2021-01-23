# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {ListNode} head
# @param {TreeNode} root
# @return {Boolean}
def is_sub_path(head, root)
  return false if root.nil?
  return true if head.val == root.val && check_path(head, root)

  is_sub_path(head, root.left) || is_sub_path(head, root.right)
end

# @param {ListNode} head
# @param {TreeNode} root
# @return {Boolean}
def check_path(head, root)
  return true if head.nil?
  return false if root.nil? || head.val != root.val

  check_path(head.next, root.left) || check_path(head.next, root.right)
end
