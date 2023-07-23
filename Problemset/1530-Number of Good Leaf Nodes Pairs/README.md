# 1530. Number of Good Leaf Nodes Pairs
Given the `root` of a binary tree and an integer `distance`. A pair of two different **leaf** nodes of a binary tree is said to be good if the length of **the shortest path** between them is less than or equal to `distance`.

Return *the number of good leaf node pairs* in the tree.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/07/09/e1.jpg)
<pre>
<strong>Input:</strong> root = [1,2,3,null,4], distance = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong> The leaf nodes of the tree are 3 and 4 and the length of the shortest path between them is 3. This is the only good pair.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/07/09/e2.jpg)
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,6,7], distance = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> The good pairs are [4,5] and [6,7] with shortest path = 2. The pair [4,6] is not good because the length of ther shortest path between them is 4.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> root = [7,1,4,6,null,5,3,null,null,null,null,null,2], distance = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only good pair is [2,5].
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> root = [100], distance = 1
<strong>Output:</strong> 0
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> root = [1,1,1], distance = 2
<strong>Output:</strong> 1
</pre>

#### Constraints:
* The number of nodes in the `tree` is in the range `[1, 2^10]`.
* Each node's value is between `[1, 100]`.
* `1 <= distance <= 10`

## Solutions (Ruby)

### 1. DFS
```Ruby
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
```
