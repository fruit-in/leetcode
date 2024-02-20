# 117. 填充每个节点的下一个右侧节点指针 II
给定一个二叉树：

```
struct Node {
  int val;
  Node *left;
  Node *right;
  Node *next;
}
```

填充它的每个 next 指针，让这个指针指向其下一个右侧节点。如果找不到下一个右侧节点，则将 next 指针设置为 `NULL` 。

初始状态下，所有 next 指针都被设置为 `NULL` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/02/15/117_sample.png)
<pre>
<strong>输入:</strong> root = [1,2,3,4,5,null,7]
<strong>输出:</strong> [1,#,2,3,#,4,5,7,#]
<strong>解释:</strong> 给定二叉树如图 A 所示，你的函数应该填充它的每个 next 指针，以指向其下一个右侧节点，如图 B 所示。序列化输出按层序遍历顺序（由 next 指针连接），'#' 表示每层的末尾。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> root = []
<strong>输出:</strong> []
</pre>

#### 提示:
* 树中的节点数在范围 `[0, 6000]` 内
* `-100 <= Node.val <= 100`

#### 进阶:
* 你只能使用常量级额外空间。
* 使用递归解题也符合要求，本题中递归程序的隐式栈空间不计入额外空间复杂度。

## 题解 (Python)

### 1. 题解
```Python
"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""


class Solution:
    def connect(self, root: 'Node') -> 'Node':
        parent = root

        while True:
            while parent is not None and parent.left is None and parent.right is None:
                parent = parent.next

            if parent is None:
                break

            if parent.left is not None:
                head = parent.left
                curr = head
                if parent.right is not None:
                    curr.next = parent.right
                    curr = curr.next
            else:
                head = parent.right
                curr = head

            parent = parent.next

            while True:
                while parent is not None and parent.left is None and parent.right is None:
                    parent = parent.next

                if parent is None:
                    break

                if parent.left is not None:
                    curr.next = parent.left
                    curr = curr.next
                if parent.right is not None:
                    curr.next = parent.right
                    curr = curr.next

                parent = parent.next

            parent = head

        return root
```
