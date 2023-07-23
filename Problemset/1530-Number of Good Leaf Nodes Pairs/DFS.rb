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
# @param {Integer} distance
# @return {Integer}
def count_pairs(root, distance)
  paths = leaf_paths(root)
  ret = 0

  (0...paths.size).each do |i|
    (i + 1...paths.size).each do |j|
      k = 0
      k += 1 while k < [paths[i].size, paths[j].size].min && paths[i][k] == paths[j][k]
      ret += 1 if paths[i].size + paths[j].size - 2 * k <= distance
    end
  end

  ret
end

# @param {TreeNode} root
# @return {String[]}
def leaf_paths(root)
  return [] if root.nil?
  return [''] if root.left.nil? && root.right.nil?

  leaf_paths(root.left).map { |p| 'l' + p } + leaf_paths(root.right).map { |p| 'r' + p }
end
