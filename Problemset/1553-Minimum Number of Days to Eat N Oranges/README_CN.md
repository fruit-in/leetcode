# 1553. 吃掉 N 个橘子的最少天数
厨房里总共有 `n` 个橘子，你决定每一天选择如下方式之一吃这些橘子：
* 吃掉一个橘子。
* 如果剩余橘子数 `n` 能被 2 整除，那么你可以吃掉 `n/2` 个橘子。
* 如果剩余橘子数 `n` 能被 3 整除，那么你可以吃掉 `2*(n/3)` 个橘子。

每天你只能从以上 3 种方案中选择一种方案。

请你返回吃掉所有 `n` 个橘子的最少天数。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 10
<strong>输出:</strong> 4
<strong>解释:</strong> 你总共有 10 个橘子。
第 1 天：吃 1 个橘子，剩余橘子数 10 - 1 = 9。
第 2 天：吃 6 个橘子，剩余橘子数 9 - 2*(9/3) = 9 - 6 = 3。（9 可以被 3 整除）
第 3 天：吃 2 个橘子，剩余橘子数 3 - 2*(3/3) = 3 - 2 = 1。
第 4 天：吃掉最后 1 个橘子，剩余橘子数 1 - 1 = 0。
你需要至少 4 天吃掉 10 个橘子。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 6
<strong>输出:</strong> 3
<strong>解释:</strong> 你总共有 6 个橘子。
第 1 天：吃 3 个橘子，剩余橘子数 6 - 6/2 = 6 - 3 = 3。（6 可以被 2 整除）
第 2 天：吃 2 个橘子，剩余橘子数 3 - 2*(3/3) = 3 - 2 = 1。（3 可以被 3 整除）
第 3 天：吃掉剩余 1 个橘子，剩余橘子数 1 - 1 = 0。
你至少需要 3 天吃掉 6 个橘子。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 1
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 56
<strong>输出:</strong> 6
</pre>

#### 提示:
* <code>1 <= n <= 2 * 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    @cache
    def minDays(self, n: int) -> int:
        if n <= 1:
            return n

        return 1 + min(self.minDays(n // 2) + n % 2, self.minDays(n // 3) + n % 3)
```
