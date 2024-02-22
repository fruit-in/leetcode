# 1416. 恢复数组
某个程序本来应该输出一个整数数组。但是这个程序忘记输出空格了以致输出了一个数字字符串，我们所知道的信息只有：数组中所有整数都在 `[1, k]` 之间，且数组中的数字都没有前导 0 。

给你字符串 `s` 和整数 `k` 。可能会有多种不同的数组恢复结果。

按照上述程序，请你返回所有可能输出字符串 `s` 的数组方案数。

由于数组方案数可能会很大，请你返回它对 `10^9 + 7` **取余** 后的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "1000", k = 10000
<strong>输出:</strong> 1
<strong>解释:</strong> 唯一一种可能的数组方案是 [1000]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "1000", k = 10
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在任何数组方案满足所有整数都 >= 1 且 <= 10 同时输出结果为 s 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "1317", k = 2000
<strong>输出:</strong> 8
<strong>解释:</strong> 可行的数组方案为 [1317]，[131,7]，[13,17]，[1,317]，[13,1,7]，[1,31,7]，[1,3,17]，[1,3,1,7]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "2020", k = 30
<strong>输出:</strong> 1
<strong>解释:</strong> 唯一可能的数组方案是 [20,20] 。 [2020] 不是可行的数组方案，原因是 2020 > 30 。 [2,020] 也不是可行的数组方案，因为 020 含有前导 0 。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> s = "1234567890", k = 90
<strong>输出:</strong> 34
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 只包含数字且不包含前导 0 。
* <code>1 <= k <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numberOfArrays(self, s: str, k: int) -> int:
        dp = [0] * (len(s) + 1)
        dp[0] = 1

        for i in range(len(s)):
            if s[i] == '0':
                continue

            x = int(s[i])
            j = 1

            while x <= k and i + j <= len(s):
                dp[i + j] = (dp[i + j] + dp[i]) % 1000000007
                if i + j < len(s):
                    x = x * 10 + int(s[i + j])
                j += 1

        return dp[-1]
```
