# 2507. 使用质因数之和替换后可以取到的最小值
给你一个正整数 `n` 。

请你将 `n` 的值替换为 `n` 的 **质因数** 之和，重复这一过程。

* 注意，如果 `n` 能够被某个质因数多次整除，则在求和时，应当包含这个质因数同样次数。

返回 `n` 可以取到的最小值。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 15
<strong>输出:</strong> 5
<strong>解释:</strong> 最开始，n = 15 。
15 = 3 * 5 ，所以 n 替换为 3 + 5 = 8 。
8 = 2 * 2 * 2 ，所以 n 替换为 2 + 2 + 2 = 6 。
6 = 2 * 3 ，所以 n 替换为 2 + 3 = 5 。
5 是 n 可以取到的最小值。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 最开始，n = 3 。
3 是 n 可以取到的最小值。
</pre>

#### 提示:
* <code>2 <= n <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    lpf = [0] * 100001
    for i in range(2, 100001):
        if lpf[i] == 0:
            for j in range(i, 100001, i):
                if lpf[j] == 0:
                    lpf[j] = i

    def smallestValue(self, n: int) -> int:
        while self.lpf[n] != n:
            x = n
            pfsum = 0

            while x > 1:
                pfsum += self.lpf[x]
                x //= self.lpf[x]

            if n == pfsum:
                break

            n = pfsum

        return n
```
