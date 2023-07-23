# 230. 二叉搜索树中第K小的元素
给定一个二叉搜索树，编写一个函数 `kthSmallest` 来查找其中第 **k** 个最小的元素。

#### 说明:
你可以假设 k 总是有效的，1 ≤ k ≤ 二叉搜索树元素个数。

#### 示例 1:
<pre>
<b>输入:</b> root = [3,1,4,null,2], k = 1
   3
  / \
 1   4
  \
   2
<b>输出:</b> 1
</pre>

#### 示例 2:
<pre>
<b>输入:</b> root = [5,3,6,2,4,null,null,1], k = 3
       5
      / \
     3   6
    / \
   2   4
  /
 1
<b>输出:</b> 3
</pre>

#### 进阶:
如果二叉搜索树经常被修改（插入/删除操作）并且你需要频繁地查找第 k 小的值，你将如何优化 `kthSmallest` 函数？

## 题解 (Ruby)

### 1. 中序遍历
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
# @param {Integer} k
# @return {Integer}
def kth_smallest(root, k)
    stack = []

    while true
        until root.nil?
            stack.push(root)
            root = root.left
        end

        root = stack.pop
        k -= 1

        return root.val if k == 0

        root = root.right
    end
end
```
