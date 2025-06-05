# 903. DI 序列的有效排列
给定一个长度为 `n` 的字符串 `s` ，其中 `s[i]` 是:
* `“D”` 意味着减少，或者
* `“I”` 意味着增加

**有效排列** 是对有 `n + 1` 个在 `[0, n]`  范围内的整数的一个排列 `perm` ，使得对所有的 `i`：
* 如果 `s[i] == 'D'`，那么 `perm[i] > perm[i+1]`，以及；
* 如果 `s[i] == 'I'`，那么 `perm[i] < perm[i+1]`。

返回 ***有效排列***  `perm`*的数量* 。因为答案可能很大，所以请**返回你的答案对** <code>10<sup>9</sup> + 7</code> **取余**。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "DID"
<strong>输出:</strong> 5
<strong>解释:</strong>
(0, 1, 2, 3) 的五个有效排列是：
(1, 0, 3, 2)
(2, 0, 3, 1)
(2, 1, 3, 0)
(3, 0, 2, 1)
(3, 1, 2, 0)
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "D"
<strong>输出:</strong> 1
</pre>

#### 提示:
* `n == s.length`
* `1 <= n <= 200`
* `s[i]` 不是 `'I'` 就是 `'D'`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numPermsDISequence(self, s: str) -> int:
        @cache
        def dfs(i: int, greater: int) -> int:
            if i == len(s):
                return 1

            less = len(s) - i - greater
            ret = 0

            if s[i] == 'D':
                for x in range(less):
                    ret = (ret + dfs(i + 1, less - 1 - x + greater)) % 1000000007
            else:
                for x in range(greater):
                    ret = (ret + dfs(i + 1, greater - 1 - x)) % 1000000007

            return ret

        s = "I" + s

        return dfs(0, len(s))
```
