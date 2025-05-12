# 1593. 拆分字符串使唯一子字符串的数目最大
给你一个字符串 `s` ，请你拆分该字符串，并返回拆分后唯一子字符串的最大数目。

字符串 `s` 拆分后可以得到若干 **非空子字符串** ，这些子字符串连接后应当能够还原为原字符串。但是拆分出来的每个子字符串都必须是 **唯一的** 。

注意：**子字符串** 是字符串中的一个连续字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "ababccc"
<strong>输出:</strong> 5
<strong>解释:</strong> 一种最大拆分方法为 ['a', 'b', 'ab', 'c', 'cc'] 。像 ['a', 'b', 'a', 'b', 'c', 'cc'] 这样拆分不满足题目要求，因为其中的 'a' 和 'b' 都出现了不止一次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aba"
<strong>输出:</strong> 2
<strong>解释:</strong> 一种最大拆分方法为 ['a', 'ba'] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "aa"
<strong>输出:</strong> 1
<strong>解释:</strong> 无法进一步拆分字符串。
</pre>

#### 提示:
* `1 <= s.length <= 16`
* `s` 仅包含小写英文字母

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def maxUniqueSplit(self, s: str) -> int:
        ret = 1

        def dfs(i: int, subs: Set[str]) -> None:
            nonlocal ret
            if i == len(s):
                ret = max(ret, len(subs))
            if len(subs) + len(s) - i <= ret:
                return

            for j in range(i, len(s)):
                if s[i:j + 1] not in subs:
                    dfs(j + 1, subs | {s[i:j + 1]})

        dfs(0, set())

        return ret
```
