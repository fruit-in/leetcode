# 1305. 两棵二叉搜索树中的所有元素
给你 ```root1``` 和 ```root2``` 这两棵二叉搜索树。

请你返回一个列表，其中包含 **两棵树** 中的所有整数并按 **升序** 排序。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/12/29/q2-e1.png)
<pre>
<strong>输入:</strong> root1 = [2,1,4], root2 = [1,0,3]
<strong>输出:</strong> [0,1,1,2,3,4]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> root1 = [0,-10,10], root2 = [5,1,7,0,2]
<strong>输出:</strong> [-10,0,0,1,2,5,7,10]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> root1 = [], root2 = [5,1,7,0,2]
<strong>输出:</strong> [0,1,2,5,7]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> root1 = [0,-10,10], root2 = []
<strong>输出:</strong> [-10,0,10]
</pre>

#### 示例 5:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/12/29/q2-e5-.png)
<pre>
<strong>输入:</strong> root1 = [1,null,8], root2 = [8,1]
<strong>输出:</strong> [1,1,8,8]
</pre>

#### 提示:
* 每棵树最多有 ```5000``` 个节点。
* 每个节点的值在 ```[-10^5, 10^5]``` 之间。

## 题解 (Python)

### 1. 中序遍历
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def getAllElements(self, root1: TreeNode, root2: TreeNode) -> List[int]:
        curr1, curr2 = root1, root2
        nodes1, nodes2 = [], []
        flag1, flag2 = True, True
        ret = []

        while nodes1 or curr1 or nodes2 or curr2:
            if flag1:
                while curr1:
                    nodes1.append(curr1)
                    curr1 = curr1.left
                curr1 = nodes1.pop() if nodes1 else None
            if flag2:
                while curr2:
                    nodes2.append(curr2)
                    curr2 = curr2.left
                curr2 = nodes2.pop() if nodes2 else None

            if not curr2 or (curr1 and curr1.val <= curr2.val):
                ret.append(curr1.val)
                curr1 = curr1.right
                flag1, flag2 = True, False
            else:
                ret.append(curr2.val)
                curr2 = curr2.right
                flag1, flag2 = False, True

        return ret
```
