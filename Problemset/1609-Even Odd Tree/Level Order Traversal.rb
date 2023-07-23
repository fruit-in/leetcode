# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {TreeNode} root
# @return {Boolean}
def is_even_odd_tree(root)
  even_index = true
  curr_level = [root]

  until curr_level.empty?
    prev_val = even_index ? 0 : 1_000_001
    next_level = []

    curr_level.each do |node|
      return false if even_index && (node.val.even? || prev_val >= node.val)
      return false if !even_index && (node.val.odd? || prev_val <= node.val)

      prev_val = node.val
      next_level.push(node.left) unless node.left.nil?
      next_level.push(node.right) unless node.right.nil?
    end

    even_index = !even_index
    curr_level = next_level
  end

  true
end
