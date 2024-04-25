# 131. 分割回文串
给你一个字符串 `s`，请你将 `s` 分割成一些子串，使每个子串都是 **回文串** 。返回 `s` 所有可能的分割方案。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aab"
<strong>输出:</strong> [["a","a","b"],["aa","b"]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "a"
<strong>输出:</strong> [["a"]]
</pre>

#### 提示:
* `1 <= s.length <= 16`
* `s` 仅由小写英文字母组成

## 题解 (Python)

### 1. 题解
```Python
from functools import lru_cache


class Solution:
    @lru_cache()
    def partition(self, s: str) -> List[List[str]]:
        if len(s) == 0:
            return [[]]

        ret = []

        for i in range(1, len(s) + 1):
            if s[:i] == s[:i][::-1]:
                ret.extend([[s[:i]] + x for x in self.partition(s[i:])])

        return ret
```
