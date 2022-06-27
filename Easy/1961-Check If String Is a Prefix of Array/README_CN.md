# 1961. 检查字符串是否为数组前缀
给你一个字符串 `s` 和一个字符串数组 `words` ，请你判断 `s` 是否为 `words` 的 **前缀字符串** 。

字符串 `s` 要成为 `words` 的 **前缀字符串** ，需要满足：`s` 可以由 `words` 中的前 `k`（`k` 为 **正数** ）个字符串按顺序相连得到，且 `k` 不超过 `words.length` 。

如果 `s` 是 `words` 的 **前缀字符串** ，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "iloveleetcode", words = ["i","love","leetcode","apples"]
<strong>输出:</strong> true
<strong>解释:</strong>
s 可以由 "i"、"love" 和 "leetcode" 相连得到。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "iloveleetcode", words = ["apples","i","love","leetcode"]
<strong>输出:</strong> false
<strong>解释:</strong>
数组的前缀相连无法得到 s 。
</pre>

#### 提示:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 20`
* `1 <= s.length <= 1000`
* `words[i]` 和 `s` 仅由小写英文字母组成

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def isPrefixString(self, s: str, words: List[str]) -> bool:
        if s == "":
            return True
        elif words == []:
            return False
        elif len(s) < len(words[0]):
            return False
        elif s[:len(words[0])] != words[0]:
            return False
        else:
            return self.isPrefixString(s[len(words[0]):], words[1:])
```
