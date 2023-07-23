# 1530. 好叶子节点对的数量
给你二叉树的根节点 `root` 和一个整数 `distance` 。

如果二叉树中两个 **叶** 节点之间的 **最短路径长度** 小于或者等于 `distance` ，那它们就可以构成一组 **好叶子节点对** 。

返回树中 **好叶子节点对的数量** 。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/07/26/e1.jpg)
<pre>
<strong>输入:</strong> root = [1,2,3,null,4], distance = 3
<strong>输出:</strong> 1
<strong>解释:</strong> 树的叶节点是 3 和 4 ，它们之间的最短路径的长度是 3 。这是唯一的好叶子节点对。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/07/26/e2.jpg)
<pre>
<strong>输入:</strong> root = [1,2,3,4,5,6,7], distance = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 好叶子节点对为 [4,5] 和 [6,7] ，最短路径长度都是 2 。但是叶子节点对 [4,6] 不满足要求，因为它们之间的最短路径长度为 4 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> root = [7,1,4,6,null,5,3,null,null,null,null,null,2], distance = 3
<strong>输出:</strong> 1
<strong>解释:</strong> 唯一的好叶子节点对是 [2,5] 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> root = [100], distance = 1
<strong>输出:</strong> 0
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> root = [1,1,1], distance = 2
<strong>输出:</strong> 1
</pre>

#### 提示:
* `tree` 的节点数在 `[1, 2^10]` 范围内。
* 每个节点的值都在 `[1, 100]` 之间。
* `1 <= distance <= 10`

## 题解 (Ruby)

### 1. 深度优先搜索
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
