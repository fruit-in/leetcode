# 2124. 检查是否所有 A 都在 B 之前
给你一个 **仅** 由字符 `'a'` 和 `'b'` 组成的字符串  `s` 。如果字符串中 **每个** `'a'` 都出现在 **每个** `'b'` 之前，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aaabbb"
<strong>输出:</strong> true
<strong>解释:</strong>
'a' 位于下标 0、1 和 2 ；而 'b' 位于下标 3、4 和 5 。
因此，每个 'a' 都出现在每个 'b' 之前，所以返回 true 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abab"
<strong>输出:</strong> false
<strong>解释:</strong>
存在一个 'a' 位于下标 2 ，而一个 'b' 位于下标 1 。
因此，不能满足每个 'a' 都出现在每个 'b' 之前，所以返回 false 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "bbb"
<strong>输出:</strong> true
<strong>解释:</strong>
不存在 'a' ，因此可以视作每个 'a' 都出现在每个 'b' 之前，所以返回 true 。
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `s[i]` 为 `'a'` 或 `'b'`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def checkString(self, s: str) -> bool:
        return all(s[i] <= s[i + 1] for i in range(len(s) - 1))
```
