# 1718. 构建字典序最大的可行序列
给你一个整数 `n` ，请你找到满足下面条件的一个序列：
* 整数 `1` 在序列中只出现一次。
* `2` 到 `n` 之间每个整数都恰好出现两次。
* 对于每个 `2` 到 `n` 之间的整数 `i` ，两个 `i` 之间出现的距离恰好为 `i` 。

序列里面两个数 `a[i]` 和 `a[j]` 之间的 **距离** ，我们定义为它们下标绝对值之差 `|j - i|` 。

请你返回满足上述条件中 **字典序最大** 的序列。题目保证在给定限制条件下，一定存在解。

一个序列 `a` 被认为比序列 `b` （两者长度相同）字典序更大的条件是： `a` 和 `b` 中第一个不一样的数字处，`a` 序列的数字比 `b` 序列的数字大。比方说，`[0,1,9,0]` 比 `[0,1,5,6]` 字典序更大，因为第一个不同的位置是第三个数字，且 `9` 比 `5` 大。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> [3,1,2,3,2]
<strong>解释:</strong> [2,3,2,1,3] 也是一个可行的序列，但是 [3,1,2,3,2] 是字典序最大的序列。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 5
<strong>输出:</strong> [5,3,1,4,3,5,2,4,2]
</pre>

#### 提示:
* `1 <= n <= 20`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def constructDistancedSequence(self, n: int) -> List[int]:
        def dfs(i: int, mask: int) -> bool:
            if i == len(a):
                return mask == (1 << (n + 1)) - 2
            if a[i] > 0:
                return dfs(i + 1, mask)

            for x in range(n, 0, -1):
                if (mask >> x) & 1 == 0 and (x == 1 or (i + x < len(a) and a[i + x] == 0)):
                    a[i] = x
                    if x > 1:
                        a[i + x] = x
                    if dfs(i + 1, mask | (1 << x)):
                        return True
                    a[i] = 0
                    if x > 1:
                        a[i + x] = 0

            return False

        a = [0] * (n * 2 - 1)
        dfs(0, 0)

        return a
```
