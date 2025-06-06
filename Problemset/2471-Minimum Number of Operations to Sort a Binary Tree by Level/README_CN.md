# 2471. 逐层排序二叉树所需的最少操作数目
给你一个 **值互不相同** 的二叉树的根节点 `root` 。

在一步操作中，你可以选择 **同一层** 上任意两个节点，交换这两个节点的值。

返回每一层按 **严格递增顺序** 排序所需的最少操作数目。

节点的 **层数** 是该节点和根节点之间的路径的边数。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/09/18/image-20220918174006-2.png)
<pre>
<strong>输入:</strong> root = [1,4,3,7,6,8,5,null,null,null,null,9,null,10]
<strong>输出:</strong> 3
<strong>解释:</strong>
- 交换 4 和 3 。第 2 层变为 [3,4] 。
- 交换 7 和 5 。第 3 层变为 [5,6,8,7] 。
- 交换 8 和 7 。第 3 层变为 [5,6,7,8] 。
共计用了 3 步操作，所以返回 3 。
可以证明 3 是需要的最少操作数目。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/09/18/image-20220918174026-3.png)
<pre>
<strong>输入:</strong> root = [1,3,2,7,6,5,4]
<strong>输出:</strong> 3
<strong>解释:</strong>
- 交换 3 和 2 。第 2 层变为 [2,3] 。
- 交换 7 和 4 。第 3 层变为 [4,6,5,7] 。
- 交换 6 和 5 。第 3 层变为 [4,5,6,7] 。
共计用了 3 步操作，所以返回 3 。
可以证明 3 是需要的最少操作数目。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2022/09/18/image-20220918174052-4.png)
<pre>
<strong>输入:</strong> root = [1,2,3,4,5,6]
<strong>输出:</strong> 0
<strong>解释:</strong> 每一层已经按递增顺序排序，所以返回 0 。
</pre>

#### 提示:
* 树中节点的数目在范围 <code>[1, 10<sup>5</sup>]</code> 。
* <code>1 <= Node.val <= 10<sup>5</sup></code>
* 树中的所有值 **互不相同** 。

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
    def minimumOperations(self, root: Optional[TreeNode]) -> int:
        nodes = [root]
        ret = 0

        while nodes:
            nextlevel = []
            vals = []
            heap = []

            for node in nodes:
                if node.left:
                    nextlevel.append(node.left)
                    vals.append(node.left.val)
                    heapq.heappush(heap, (vals[-1], len(vals) - 1))
                if node.right:
                    nextlevel.append(node.right)
                    vals.append(node.right.val)
                    heapq.heappush(heap, (vals[-1], len(vals) - 1))

            for i in range(len(vals)):
                while heap[0][0] != vals[heap[0][1]]:
                    heapq.heappop(heap)

                j = heapq.heappop(heap)[1]
                if i != j:
                    heapq.heappush(heap, (vals[i], j))
                    vals[i], vals[j] = vals[j], vals[i]
                    ret += 1

            nodes = nextlevel

        return ret
```
