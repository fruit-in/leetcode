# 138. 复制带随机指针的链表
给定一个链表，每个节点包含一个额外增加的随机指针，该指针可以指向链表中的任何节点或空节点。

要求返回这个链表的 **[深拷贝](https://baike.baidu.com/item/%E6%B7%B1%E6%8B%B7%E8%B4%9D/22785317?fr=aladdin)**。

我们用一个由 `n` 个节点组成的链表来表示输入/输出中的链表。每个节点用一个 `[val, random_index]` 表示：
* `val`：一个表示 `Node.val` 的整数。
* `random_index`：随机指针指向的节点索引（范围从 `0` 到 `n-1`）；如果不指向任何节点，则为  `null` 。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/09/e1.png)
<pre>
<strong>输入:</strong> head = [[7,null],[13,0],[11,4],[10,2],[1,0]]
<strong>输出:</strong> [[7,null],[13,0],[11,4],[10,2],[1,0]]
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/09/e2.png)
<pre>
<strong>输入:</strong> head = [[1,1],[2,1]]
<strong>输出:</strong> [[1,1],[2,1]]
</pre>

#### 示例 3:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/09/e3.png)
<pre>
<strong>输入:</strong> head = [[3,null],[3,0],[3,null]]
<strong>输出:</strong> [[3,null],[3,0],[3,null]]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> head = []
<strong>输出:</strong> []
<strong>解释:</strong> 给定的链表为空（空指针），因此返回 null。
</pre>

#### 提示:
* `-10000 <= Node.val <= 10000`
* `Node.random` 为空（null）或指向链表中的节点。
* 节点数目不超过 1000 。

## 题解 (Python)

### 1. 题解
```Python
"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: 'Node') -> 'Node':
        if not head:
            return None

        curr = head

        while curr:
            copy = Node(curr.val, curr.next, curr.random)
            curr.next = copy
            curr = copy.next

        curr = head.next

        while curr:
            curr.next = curr.next.next if curr.next else None
            curr.random = curr.random.next if curr.random else None
            curr = curr.next

        return head.next
```
