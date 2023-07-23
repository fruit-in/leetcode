# 1957. 删除字符使字符串变好
一个字符串如果没有 **三个连续** 相同字符，那么它就是一个 **好字符串** 。

给你一个字符串 `s` ，请你从 `s` 删除 **最少** 的字符，使它变成一个 **好字符串** 。

请你返回删除后的字符串。题目数据保证答案总是 **唯一的** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "leeetcode"
<strong>输出:</strong> "leetcode"
<strong>解释:</strong>
从第一组 'e' 里面删除一个 'e' ，得到 "leetcode" 。
没有连续三个相同字符，所以返回 "leetcode" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aaabaaaa"
<strong>输出:</strong> "aabaa"
<strong>解释:</strong>
从第一组 'a' 里面删除一个 'a' ，得到 "aabaaaa" 。
从第二组 'a' 里面删除两个 'a' ，得到 "aabaa" 。
没有连续三个相同字符，所以返回 "aabaa" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "aab"
<strong>输出:</strong> "aab"
<strong>解释:</strong> 没有连续三个相同字符，所以返回 "aab"
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 只包含小写英文字母。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def makeFancyString(self, s: str) -> str:
        ret = ""

        for c in s:
            if len(ret) < 2 or ret[-1] != c or ret[-2] != c:
                ret += c

        return ret
```

## Solutions (Language)

### 1. Solution
```Language
```
