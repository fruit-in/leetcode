# 2196. 根据描述创建二叉树
给你一个二维整数数组 `descriptions` ，其中 <code>descriptions[i] = [parent<sup>i</sup>, child<sup>i</sup>, isLeft<sup>i</sup>]</code> 表示 <code>parent<sup>i</sup></code> 是 <code>child<sup>i</sup></code> 在 **二叉树** 中的 **父节点**，二叉树中各节点的值 **互不相同** 。此外：

* 如果 <code>isLeft<sup>i</sup></code> == 1 ，那么 <code>child<sup>i</sup></code> 就是 <code>parent<sup>i</sup></code> 的左子节点。
* 如果 <code>isLeft<sup>i</sup></code> == 0 ，那么 <code>child<sup>i</sup></code> 就是 <code>parent<sup>i</sup></code> 的右子节点。

请你根据 `descriptions` 的描述来构造二叉树并返回其 **根节点** 。

测试用例会保证可以构造出 **有效** 的二叉树。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/02/09/example1drawio.png)
<pre>
<strong>输入:</strong> descriptions = [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
<strong>输出:</strong> [50,20,80,15,17,19]
<strong>解释:</strong> 根节点是值为 50 的节点，因为它没有父节点。
结果二叉树如上图所示。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/02/09/example2drawio.png)
<pre>
<strong>输入:</strong> descriptions = [[1,2,1],[2,3,0],[3,4,1]]
<strong>输出:</strong> [1,2,null,null,3,4]
<strong>解释:</strong> 根节点是值为 1 的节点，因为它没有父节点。
结果二叉树如上图所示。
</pre>

#### 提示:
* <code>1 <= descriptions.length <= 10<sup>4</sup></code>
* `descriptions[i].length == 3`
* <code>1 <= parent<sup>i</sup>, child<sup>i</sup> <= 10<sup>5</sup></code>
* <code>0 <= isLeft<sup>i</sup> <= 1</code>
* `descriptions` 所描述的二叉树是一棵有效二叉树

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
    def createBinaryTree(self, descriptions: List[List[int]]) -> Optional[TreeNode]:
        parents = set()
        children = set()
        nodes = {}

        for parent, child, isleft in descriptions:
            parents.add(parent)
            children.add(child)

            if parent not in nodes:
                nodes[parent] = TreeNode(parent)
            if child not in nodes:
                nodes[child] = TreeNode(child)

            if isleft:
                nodes[parent].left = nodes[child]
            else:
                nodes[parent].right = nodes[child]

        return nodes[(parents - children).pop()]
```
