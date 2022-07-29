# 829. 连续整数求和
给定一个正整数 `n`，返回 *连续正整数满足所有数字之和为 `n` 的组数* 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 5
<strong>输出:</strong> 2
<strong>解释:</strong> 5 = 2 + 3，共有两组连续整数([5],[2,3])求和后为 5。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 9
<strong>输出:</strong> 3
<strong>解释:</strong> 9 = 4 + 5 = 2 + 3 + 4
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 15
<strong>输出:</strong> 4
<strong>解释:</strong> 15 = 8 + 7 = 4 + 5 + 6 = 1 + 2 + 3 + 4 + 5
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def consecutiveNumbersSum(self, n: int) -> int:
        ret = 0

        for x in range(1, int((2 * n) ** .5) + 1):
            if (n + (x - 1) * x // 2) % x == 0 and (n + (x - 1) * x // 2) // x > 0:
                ret += 1

        return ret
```
