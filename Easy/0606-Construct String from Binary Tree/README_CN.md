# 606. 根据二叉树创建字符串
你需要采用前序遍历的方式，将一个二叉树转换成一个由括号和整数组成的字符串。

空节点则用一对空括号 "()" 表示。而且你需要省略所有不影响字符串与原始二叉树之间的一对一映射关系的空括号对。

#### 示例 1:
<pre>
<strong>输入:</strong> 二叉树: [1,2,3,4]
       1
     /   \
    2     3
   /    
  4     
<strong>输出:</strong> "1(2(4))(3)"
<strong>解释:</strong> 原本将是“1(2(4)())(3())”，
在你省略所有不必要的空括号对之后，
它将是“1(2(4))(3)”。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 二叉树: [1,2,3,null,4]
       1
     /   \
    2     3
     \  
      4 
<strong>输出:</strong> "1(2()(4))(3)"
<strong>解释:</strong> 和第一个示例相似，
除了我们不能省略第一个对括号来中断输入和输出之间的一对一映射关系。
</pre>

## 题解 (Python)

### 1. 递归
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def tree2str(self, t: TreeNode) -> str:
        if not t:
            return ""

        ret = str(t.val)

        if t.left or t.right:
            ret += "(" + self.tree2str(t.left) + ")"
        if t.right:
            ret += "(" + self.tree2str(t.right) + ")"

        return ret
```

### 2. 迭代
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def tree2str(self, t: TreeNode) -> str:
        if not t:
            return ""

        ret = ""
        nodes = [t]
        visited = set()

        while nodes:
            cur = nodes.pop()
            if cur not in visited:
                visited.add(cur)
                nodes.append(cur)
                ret += "(" + str(cur.val)

                if cur.right:
                    nodes.append(cur.right)
                if cur.left:
                    nodes.append(cur.left)
                if not cur.left and cur.right:
                    ret += "()"
            else:
                ret += ")"

        return ret[1:-1]
```
