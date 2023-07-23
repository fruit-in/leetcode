# 524. 通过删除字母匹配到字典里最长单词
给你一个字符串 `s` 和一个字符串数组 `dictionary` ，找出并返回 `dictionary` 中最长的字符串，该字符串可以通过删除 `s` 中的某些字符得到。

如果答案不止一个，返回长度最长且字母序最小的字符串。如果答案不存在，则返回空字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abpcplea", dictionary = ["ale","apple","monkey","plea"]
<strong>输出:</strong> "apple"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abpcplea", dictionary = ["a","b","c"]
<strong>输出:</strong> "a"
</pre>

#### 提示:
* `1 <= s.length <= 1000`
* `1 <= dictionary.length <= 1000`
* `1 <= dictionary[i].length <= 1000`
* `s` 和 `dictionary[i]` 仅由小写英文字母组成

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findLongestWord(self, s: str, dictionary: List[str]) -> str:
        for word in sorted(dictionary, key=lambda word: (-len(word), word)):
            i = 0

            for j in range(len(s)):
                if word[i] == s[j]:
                    i += 1

                if i == len(word):
                    return word

        return ""
```
