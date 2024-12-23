# 863. 二叉树中所有距离为 K 的结点
给定一个二叉树（具有根结点 `root`）， 一个目标结点 `target` ，和一个整数值 `k` ，返回到目标结点 `target` 距离为 `k` 的所有结点的值的数组。

答案可以以 **任何顺序** 返回。

#### 示例 1:
![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/28/sketch0.png)
<pre>
<strong>输入:</strong> root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
<strong>输出:</strong> [7,4,1]
<strong>解释:</strong> 所求结点为与目标结点（值为 5）距离为 2 的结点，值分别为 7，4，以及 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> root = [1], target = 1, k = 3
<strong>输出:</strong> []
</pre>

#### 提示:
* 节点数在 `[1, 500]` 范围内
* `0 <= Node.val <= 500`
* `Node.val` 中所有值 **不同**
* 目标结点 `target` 是树上的结点。
* `0 <= k <= 1000`

## 题解 (Python)

### 1. 题解
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def distanceK(self, root: TreeNode, target: TreeNode, k: int) -> List[int]:
        root.parent = None
        stack = [root]
        deque = collections.deque([(target, 0)])
        ret = []

        while stack != []:
            node = stack.pop()
            if node.left is not None:
                node.left.parent = node
                stack.append(node.left)
            if node.right is not None:
                node.right.parent = node
                stack.append(node.right)

        while len(deque) > 0:
            node, d = deque.popleft()

            if d > k:
                break
            elif d == k:
                ret.append(node.val)

            if node.left is not None:
                deque.append((node.left, d + 1))
                node.left.parent = None
            if node.right is not None:
                deque.append((node.right, d + 1))
                node.right.parent = None
            if node.parent is not None:
                deque.append((node.parent, d + 1))
                if node == node.parent.left:
                    node.parent.left = None
                else:
                    node.parent.right = None

        return ret
```
