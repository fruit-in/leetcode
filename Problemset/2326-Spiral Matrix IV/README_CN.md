# 2326. 螺旋矩阵 IV
给你两个整数：`m` 和 `n` ，表示矩阵的维数。

另给你一个整数链表的头节点 `head` 。

请你生成一个大小为 `m x n` 的螺旋矩阵，矩阵包含链表中的所有整数。链表中的整数从矩阵 **左上角** 开始、**顺时针** 按 **螺旋** 顺序填充。如果还存在剩余的空格，则用 `-1` 填充。

返回生成的矩阵。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/05/09/ex1new.jpg)
<pre>
<strong>输入:</strong> m = 3, n = 5, head = [3,0,2,6,8,1,7,9,4,2,5,5,0]
<strong>输出:</strong> [[3,0,2,6,8],[5,0,-1,-1,1],[5,2,4,9,7]]
<strong>解释:</strong> 上图展示了链表中的整数在矩阵中是如何排布的。
注意，矩阵中剩下的空格用 -1 填充。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/05/11/ex2.jpg)
<pre>
<strong>输入:</strong> m = 1, n = 4, head = [0,1,2]
<strong>输出:</strong> [[0,1,2,-1]]
<strong>解释:</strong> 上图展示了链表中的整数在矩阵中是如何从左到右排布的。
注意，矩阵中剩下的空格用 -1 填充。
</pre>

#### 提示:
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>1 <= m * n <= 10<sup>5</sup></code>
* 链表中节点数目在范围 `[1, m * n]` 内
* `0 <= Node.val <= 1000`

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def spiralMatrix(self, m: int, n: int, head: Optional[ListNode]) -> List[List[int]]:
        curr = head
        i, j = 0, -1
        ret = [[-1] * n for _ in range(m)]

        while curr is not None:
            for (a, b) in [(0, 1)] * n + [(1, 0)] * (m - 1) + [(0, -1)] * (n - 1) + [(-1, 0)] * (m - 2):
                if curr is None:
                    break

                i += a
                j += b
                ret[i][j] = curr.val
                curr = curr.next

            m -= 2
            n -= 2

        return ret
```
