# 297. 二叉树的序列化与反序列化
序列化是将一个数据结构或者对象转换为连续的比特位的操作，进而可以将转换后的数据存储在一个文件或者内存中，同时也可以通过网络传输到另一个计算机环境，采取相反方式重构得到原数据。

请设计一个算法来实现二叉树的序列化与反序列化。这里不限定你的序列 / 反序列化算法执行逻辑，你只需要保证一个二叉树可以被序列化为一个字符串并且将这个字符串反序列化为原始的树结构。

**提示:** 输入输出格式与 LeetCode 目前使用的方式一致，详情请参阅 [LeetCode 序列化二叉树的格式](https://support.leetcode.cn/hc/kb/article/1567641/)。你并非必须采取这种方式，你也可以采用其他的方法解决这个问题。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/15/serdeser.jpg)
<pre>
<strong>输入:</strong> root = [1,2,3,null,null,4,5]
<strong>输出:</strong> [1,2,3,null,null,4,5]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> root = []
<strong>输出:</strong> []
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> root = [1]
<strong>输出:</strong> [1]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> root = [1,2]
<strong>输出:</strong> [1,2]
</pre>

#### 提示:
* 树中结点数在范围 <code>[0, 10<sup>4</sup>]</code> 内
* `-1000 <= Node.val <= 1000`

## 题解 (Python)

### 1. 题解
```Python
# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Codec:

    def serialize(self, root):
        """Encodes a tree to a single string.

        :type root: TreeNode
        :rtype: str
        """
        nodes = [root]
        i = 0

        while i < len(nodes):
            if nodes[i] is not None:
                nodes.append(nodes[i].left)
                nodes.append(nodes[i].right)

            i += 1

        return ','.join('N' if node is None else str(node.val) for node in nodes)

    def deserialize(self, data):
        """Decodes your encoded data to tree.

        :type data: str
        :rtype: TreeNode
        """
        nodes = [TreeNode(int(x)) if x !=
                 'N' else None for x in data.split(',')]
        i = 0
        j = 1

        while j < len(nodes):
            nodes[i].left = nodes[j]
            nodes[i].right = nodes[j + 1]
            i += 1
            j += 2
            while i < j and nodes[i] is None:
                i += 1

        return nodes[0]

# Your Codec object will be instantiated and called as such:
# ser = Codec()
# deser = Codec()
# ans = deser.deserialize(ser.serialize(root))
```
