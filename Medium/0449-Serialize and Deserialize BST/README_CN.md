# 449. 序列化和反序列化二叉搜索树
序列化是将数据结构或对象转换为一系列位的过程，以便它可以存储在文件或内存缓冲区中，或通过网络连接链路传输，以便稍后在同一个或另一个计算机环境中重建。

设计一个算法来序列化和反序列化 **二叉搜索树** 。 对序列化/反序列化算法的工作方式没有限制。 您只需确保二叉搜索树可以序列化为字符串，并且可以将该字符串反序列化为最初的二叉搜索树。

**编码的字符串应尽可能紧凑。**

#### 示例 1:
<pre>
<strong>输入:</strong> root = [2,1,3]
<strong>输出:</strong> [2,1,3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> root = []
<strong>输出:</strong> []
</pre>

#### 提示:
* 树中节点数范围是 <code>[0, 10<sup>4</sup>]</code>.
* <code>0 <= Node.val <= 10<sup>4</sup></code>
* 题目数据 **保证** 输入的树是一棵二叉搜索树。

**注意:** 不要使用类成员/全局/静态变量来存储状态。 你的序列化和反序列化算法应该是无状态的。

## 题解 (Python)

### 1. 递归
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Codec:

    def serialize(self, root: TreeNode) -> str:
        if not root:
            return ""

        return chr(root.val) \
            + self.serialize(root.left) + self.serialize(root.right)

    def deserialize(self, data: str) -> TreeNode:
        if not data:
            return None

        i = next((i for i in range(len(data)) if data[i] > data[0]), len(data))

        root = TreeNode(ord(data[0]))
        root.left = self.deserialize(data[1:i])
        root.right = self.deserialize(data[i:])

        return root


# Your Codec object will be instantiated and called as such:
# Your Codec object will be instantiated and called as such:
# ser = Codec()
# deser = Codec()
# tree = ser.serialize(root)
# ans = deser.deserialize(tree)
# return ans
```
