# 1110. 删点成林
给出二叉树的根节点 `root`，树上每个节点都有一个不同的值。

如果节点值在 `to_delete` 中出现，我们就把该节点从树上删去，最后得到一个森林（一些不相交的树构成的集合）。

返回森林中的每棵树。你可以按任意顺序组织答案。

#### 示例:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/07/05/screen-shot-2019-07-01-at-53836-pm.png)
<pre>
<strong>输入:</strong> root = [1,2,3,4,5,6,7], to_delete = [3,5]
<strong>输出:</strong> [[1,2,null,4],[6],[7]]
</pre>

#### 提示:
* 树中的节点数最大为 `1000`。
* 每个节点都有一个介于 `1` 到 `1000` 之间的值，且各不相同。
* `to_delete.length <= 1000`
* `to_delete` 包含一些从 `1` 到 `1000`、各不相同的值。

## 题解 (Python)

### 1. 题解
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def delNodes(self, root: TreeNode, to_delete: List[int]) -> List[TreeNode]:
        if root is None:
            return []

        ret = [root]
        left_ret = self.delNodes(root.left, to_delete)
        right_ret = self.delNodes(root.right, to_delete)

        if root.val in to_delete:
            return left_ret + right_ret

        if root.left is not None:
            if root.left.val in to_delete:
                root.left = None
                ret += left_ret
            else:
                ret += left_ret[1:]
        if root.right is not None:
            if root.right.val in to_delete:
                root.right = None
                ret += right_ret
            else:
                ret += right_ret[1:]

        return ret
```

## 题解 (Ruby)

### 1. 题解
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
# @param {Integer[]} to_delete
# @return {TreeNode[]}
def del_nodes(root, to_delete)
  return [] if root.nil?

  ret = [root]
  left_ret = del_nodes(root.left, to_delete)
  right_ret = del_nodes(root.right, to_delete)

  return left_ret + right_ret if to_delete.include?(root.val)

  unless root.left.nil?
    if to_delete.include?(root.left.val)
      root.left = nil
      ret += left_ret
    else
      ret += left_ret[1..]
    end
  end
  unless root.right.nil?
    if to_delete.include?(root.right.val)
      root.right = nil
      ret += right_ret
    else
      ret += right_ret[1..]
    end
  end

  ret
end
```
