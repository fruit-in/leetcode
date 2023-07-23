# 337. 打家劫舍 III
在上次打劫完一条街道之后和一圈房屋后，小偷又发现了一个新的可行窃的地区。这个地区只有一个入口，我们称之为“根”。 除了“根”之外，每栋房子有且只有一个“父“房子与之相连。一番侦察之后，聪明的小偷意识到“这个地方的所有房屋的排列类似于一棵二叉树”。 如果两个直接相连的房子在同一天晚上被打劫，房屋将自动报警。

计算在不触动警报的情况下，小偷一晚能够盗取的最高金额。

#### 示例 1:
<pre>
<strong>输入:</strong> [3,2,3,null,3,null,1]

     <b>3</b>
    / \
   2   3
    \   \
     <b>3   1</b>
<strong>输出:</strong> 7
<strong>解释:</strong> 小偷一晚能够盗取的最高金额 = <b>3</b> + <b>3</b> + <b>1</b> = <b>7</b>.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [3,4,5,1,3,null,1]

     3
    / \
   <b>4   5</b>
  / \   \
 1   3   1
<strong>输出:</strong> 9
<strong>解释:</strong> 小偷一晚能够盗取的最高金额 = <b>4</b> + <b>5</b> = <b>9</b>.
</pre>

## 题解 (Python)

### 1. 递归
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def rob(self, root: TreeNode) -> int:
        def foo(root: TreeNode) -> (int, int):
            if not root:
                return (0, 0)

            rob_left = foo(root.left)
            rob_right = foo(root.right)

            return (root.val + rob_left[1] + rob_right[1],
                    max(rob_left) + max(rob_right))

        return max(foo(root))
```

## 题解 (Ruby)

### 1. 递归
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
# @return {Integer}
def rob(root)
  foo(root).max
end

# @param {TreeNode} root
# @return {Integer[]}
def foo(root)
  return [0, 0] if root.nil?

  rob_left = foo(root.left)
  rob_right = foo(root.right)

  [root.val + rob_left[1] + rob_right[1], rob_left.max + rob_right.max]
end
```
