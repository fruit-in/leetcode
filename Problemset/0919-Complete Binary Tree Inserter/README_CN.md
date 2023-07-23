# 919. 完全二叉树插入器
**完全二叉树** 是每一层（除最后一层外）都是完全填充（即，节点数达到最大）的，并且所有的节点都尽可能地集中在左侧。

设计一种算法，将一个新节点插入到一个完整的二叉树中，并在插入后保持其完整。

实现 `CBTInserter` 类:
* `CBTInserter(TreeNode root)` 使用头节点为 `root` 的给定树初始化该数据结构；
* `CBTInserter.insert(int v)`  向树中插入一个值为 `Node.val == val`的新节点 `TreeNode`。使树保持完全二叉树的状态，**并返回插入节点** `TreeNode` **的父节点的值**；
* `CBTInserter.get_root()` 将返回树的头节点。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/08/03/lc-treeinsert.jpg)
<pre>
<strong>输入:</strong>
["CBTInserter", "insert", "insert", "get_root"]
[[[1, 2]], [3], [4], []]
<strong>输出:</strong>
[null, 1, 2, [1, 2, 3, 4]]
<strong>解释:</strong>
CBTInserter cBTInserter = new CBTInserter([1, 2]);
cBTInserter.insert(3);  // 返回 1
cBTInserter.insert(4);  // 返回 2
cBTInserter.get_root(); // 返回 [1, 2, 3, 4]
</pre>

#### 提示:
* 树中节点数量范围为 `[1, 1000]`
* `0 <= Node.val <= 5000`
* `root` 是完全二叉树
* `0 <= val <= 5000`
* 每个测试用例最多调用 `insert` 和 `get_root` 操作 <code>10<sup>4</sup></code> 次

## 题解 (Python)

### 1. 题解
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class CBTInserter:

    def __init__(self, root: Optional[TreeNode]):
        self.root = root
        self.size = 0
        stack = [root]

        while stack:
            node = stack.pop()

            if node:
                self.size += 1
                stack.append(node.left)
                stack.append(node.right)

    def insert(self, val: int) -> int:
        self.size += 1
        size = self.size
        node = self.root
        stack = []

        while size > 1:
            stack.append(size & 1)
            size >>= 1

        while len(stack) > 1:
            node = node.left if stack.pop() == 0 else node.right

        if stack.pop() == 0:
            node.left = TreeNode(val)
        else:
            node.right = TreeNode(val)

        return node.val

    def get_root(self) -> Optional[TreeNode]:
        return self.root

# Your CBTInserter object will be instantiated and called as such:
# obj = CBTInserter(root)
# param_1 = obj.insert(val)
# param_2 = obj.get_root()
```
