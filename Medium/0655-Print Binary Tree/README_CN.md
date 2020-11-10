# 655. 输出二叉树
在一个 m*n 的二维字符串数组中输出二叉树，并遵守以下规则：
1. 行数 `m` 应当等于给定二叉树的高度。
2. 列数 `n` 应当总是奇数。
3. 根节点的值（以字符串格式给出）应当放在可放置的第一行正中间。根节点所在的行与列会将剩余空间划分为两部分（**左下部分和右下部分**）。你应该将左子树输出在左下部分，右子树输出在右下部分。左下和右下部分应当有相同的大小。即使一个子树为空而另一个非空，你不需要为空的子树输出任何东西，但仍需要为另一个子树留出足够的空间。然而，如果两个子树都为空则不需要为它们留出任何空间。
4. 每个未使用的空间应包含一个空的字符串`""`。
5. 使用相同的规则输出子树。

#### 示例 1:
<pre>
<b>输入:</b>
     1
    /
   2
<b>输出:</b>
[["", "1", ""],
 ["2", "", ""]]
</pre>

#### 示例 2:
<pre>
<b>输入:</b>
     1
    / \
   2   3
    \
     4
<b>输出:</b>
[["", "", "", "1", "", "", ""],
 ["", "2", "", "", "", "3", ""],
 ["", "", "4", "", "", "", ""]]
</pre>

#### 示例 3:
<pre>
<b>输入:</b>
      1
     / \
    2   5
   /
  3
 /
4
<b>输出:</b>
[["",  "",  "", "",  "", "", "", "1", "",  "",  "",  "",  "", "", ""]
 ["",  "",  "", "2", "", "", "", "",  "",  "",  "",  "5", "", "", ""]
 ["",  "3", "", "",  "", "", "", "",  "",  "",  "",  "",  "", "", ""]
 ["4", "",  "", "",  "", "", "", "",  "",  "",  "",  "",  "", "", ""]]
</pre>

#### 注意:
二叉树的高度在范围 [1, 10] 中。

## 题解 (Python)

### 1. 层序遍历
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def printTree(self, root: TreeNode) -> List[List[str]]:
        curr_level = [root]
        vals = []

        while any(curr_level):
            curr_vals = []
            next_level = []

            for node in curr_level:
                if node:
                    curr_vals.append(str(node.val))
                    next_level.append(node.left)
                    next_level.append(node.right)
                else:
                    curr_vals.append("")
                    next_level.append(None)
                    next_level.append(None)

            vals.append(curr_vals)
            curr_level = next_level

        indices = [i for i in range(0, 2 ** len(vals), 2)]
        ret = [[""] * (2 ** len(vals) - 1) for _ in range(len(vals))]

        for i in range(-1, -1 - len(vals), -1):
            new_indices = []

            while indices:
                index0 = indices.pop()
                ret[i][index0] = vals[i].pop()
                if indices:
                    index1 = indices.pop()
                    ret[i][index1] = vals[i].pop()
                    new_indices.append((index0 + index1) // 2)

            indices = new_indices[::-1]

        return ret
```
