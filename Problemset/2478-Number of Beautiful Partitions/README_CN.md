# 2478. 完美分割的方案数
给你一个字符串 `s` ，每个字符是数字 `'1'` 到 `'9'` ，再给你两个整数 `k` 和 `minLength` 。

如果对 `s` 的分割满足以下条件，那么我们认为它是一个 **完美** 分割：
* `s` 被分成 `k` 段互不相交的子字符串。
* 每个子字符串长度都 **至少** 为 `minLength` 。
* 每个子字符串的第一个字符都是一个 **质数** 数字，最后一个字符都是一个 **非质数** 数字。质数数字为 `'2'` ，`'3'` ，`'5'` 和 `'7'` ，剩下的都是非质数数字。

请你返回 `s` 的 **完美** 分割数目。由于答案可能很大，请返回答案对 <code>10<sup>9</sup> + 7</code> **取余** 后的结果。

一个 **子字符串** 是字符串中一段连续字符串序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "23542185131", k = 3, minLength = 2
<strong>输出:</strong> 3
<strong>解释:</strong> 存在 3 种完美分割方案：
"2354 | 218 | 5131"
"2354 | 21851 | 31"
"2354218 | 51 | 31"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "23542185131", k = 3, minLength = 3
<strong>输出:</strong> 1
<strong>解释:</strong> 存在一种完美分割方案："2354 | 218 | 5131" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "3312958", k = 3, minLength = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 存在一种完美分割方案："331 | 29 | 58" 。
</pre>

#### 提示:
* `1 <= k, minLength <= s.length <= 1000`
* `s` 每个字符都为数字 `'1'` 到 `'9'` 之一。

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    def beautifulPartitions(self, s: str, k: int, minLength: int) -> int:
        @cache
        def subPartitions(i: int, k: int) -> int:
            if k == 0:
                return i == len(s)
            if len(s) - i < k * minLength or s[i] not in "2357":
                return 0

            ret = 0

            for j in range(i + minLength, len(s) - (k - 1) * minLength + 1):
                if s[j - 1] not in "2357":
                    ret = (ret + subPartitions(j, k - 1)) % 1000000007

            return ret

        if s[-1] in "2357":
            return 0

        return subPartitions(0, k)
```
