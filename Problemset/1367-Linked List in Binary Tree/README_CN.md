# 1367. 二叉树中的列表
给你一棵以 `root` 为根的二叉树和一个 `head` 为第一个节点的链表。

如果在二叉树中，存在一条一直向下的路径，且每个点的数值恰好一一对应以 `head` 为首的链表中每个节点的值，那么请你返回 `True` ，否则返回 `False` 。

一直向下的路径的意思是：从树中某个节点开始，一直连续向下的路径。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/02/29/sample_1_1720.png)
<pre>
<strong>输入:</strong> head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
<strong>输出:</strong> true
<strong>解释:</strong> 树中蓝色的节点构成了与链表对应的子路径。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/02/29/sample_2_1720.png)
<pre>
<strong>输入:</strong> head = [1,4,2,6], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
<strong>输出:</strong> true
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> head = [1,4,2,6,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
<strong>输出:</strong> false
<strong>解释:</strong> 二叉树中不存在一一对应链表的路径。
</pre>

#### 提示:
* 二叉树和链表中的每个节点的值都满足 `1 <= node.val <= 100` 。
* 链表包含的节点数目在 `1` 到 `100` 之间。
* 二叉树包含的节点数目在 `1` 到 `2500` 之间。

## 题解 (Python)

### 1. 递归
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isSubPath(self, head: ListNode, root: TreeNode) -> bool:
        if root is None:
            return False
        elif head.val == root.val and self.checkPath(head, root):
            return True

        return self.isSubPath(head, root.left) or self.isSubPath(head, root.right)

    def checkPath(self, head: ListNode, root: TreeNode) -> bool:
        if head is None:
            return True
        elif root is None or head.val != root.val:
            return False

        return self.checkPath(head.next, root.left) or self.checkPath(head.next, root.right)
```

## 题解 (Ruby)

### 1. 递归
```Ruby
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
```
