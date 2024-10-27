# 2385. 感染二叉树需要的总时间
给你一棵二叉树的根节点 `root` ，二叉树中节点的值 **互不相同** 。另给你一个整数 `start` 。在第 `0` 分钟，**感染** 将会从值为 `start` 的节点开始爆发。

每分钟，如果节点满足以下全部条件，就会被感染：

* 节点此前还没有感染。
* 节点与一个已感染节点相邻。

返回感染整棵树需要的分钟数。
You are given the `root` of a binary tree with **unique** values, and an integer `start`. At minute `0`, an **infection** starts from the node with value `start`.

Each minute, a node becomes infected if:

* The node is currently uninfected.
* The node is adjacent to an infected node.

Return *the number of minutes needed for the entire tree to be infected*.

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/06/25/image-20220625231744-1.png)
<pre>
<strong>输入:</strong> root = [1,5,3,null,4,10,6,9,2], start = 3
<strong>输出:</strong> 4
<strong>解释:</strong> The following nodes are infected during:
- Minute 0: Node 3
- Minute 1: Nodes 1, 10 and 6
- Minute 2: Node 5
- Minute 3: Node 4
- Minute 4: Nodes 9 and 2
It takes 4 minutes for the whole tree to be infected so we return 4.
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/06/25/image-20220625231812-2.png)
<pre>
<strong>输入:</strong> root = [1], start = 1
<strong>输出:</strong> 0
<strong>解释:</strong> At minute 0, the only node in the tree is infected so we return 0.
</pre>

#### 提示:
* 树中节点的数目在范围 <code>[1, 10<sup>5</sup>]</code> 内
* <code>1 <= Node.val <= 10<sup>5</sup></code>
* 每个节点的值 **互不相同**
* 树中必定存在值为 `start` 的节点

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
    def amountOfTime(self, root: Optional[TreeNode], start: int) -> int:
        def setParent(root: Optional[TreeNode]) -> None:
            if root is None:
                return

            if root.left is not None:
                root.left.parent = root
                setParent(root.left)
            if root.right is not None:
                root.right.parent = root
                setParent(root.right)

        def findNode(root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
            if root is None:
                return
            if root.val == val:
                return root
            findleft = findNode(root.left, val)
            if findleft is not None:
                return findleft
            return findNode(root.right, val)

        setParent(root)
        root.parent = None
        infected = {start}
        deque = collections.deque([(findNode(root, start), 0)])
        ret = 0

        while len(deque) > 0:
            node, minutes = deque.popleft()
            ret = minutes
            if node.parent is not None and node.parent.val not in infected:
                infected.add(node.parent.val)
                deque.append((node.parent, minutes + 1))
            if node.left is not None and node.left.val not in infected:
                infected.add(node.left.val)
                deque.append((node.left, minutes + 1))
            if node.right is not None and node.right.val not in infected:
                infected.add(node.right.val)
                deque.append((node.right, minutes + 1))

        return ret
```
