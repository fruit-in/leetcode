# 1312. 让字符串成为回文串的最少插入次数
给你一个字符串 `s` ，每一次操作你都可以在字符串的任意位置插入任意字符。

请你返回让 `s` 成为回文串的 **最少操作次数** 。

「回文串」是正读和反读都相同的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "zzazz"
<strong>输出:</strong> 0
<strong>解释:</strong> 字符串 "zzazz" 已经是回文串了，所以不需要做任何插入操作。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "mbadm"
<strong>输出:</strong> 2
<strong>解释:</strong> 字符串可变为 "mbdadbm" 或者 "mdbabdm" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "leetcode"
<strong>输出:</strong> 5
<strong>解释:</strong> 插入 5 个字符后字符串变为 "leetcodocteel" 。
</pre>

#### 提示:
* `1 <= s.length <= 500`
* `s` 中所有字符都是小写字母。

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    def minInsertions(self, s: str) -> int:
        @cache
        def minInsertionsSub(i: int, j: int) -> int:
            if i >= j:
                return 0

            ret = 1 + minInsertionsSub(i + 1, j)
            ret = min(ret, 1 + minInsertionsSub(i, j - 1))
            if s[i] == s[j]:
                ret = min(ret, minInsertionsSub(i + 1, j - 1))

            return ret

        return minInsertionsSub(0, len(s) - 1)
```
