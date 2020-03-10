# 1332. 删除回文子序列
给你一个字符串 ```s```，它仅由字母 ```'a'``` 和 ```'b'``` 组成。每一次删除操作都可以从 ```s``` 中删除一个回文 **子序列**。

返回删除给定字符串中所有字符（字符串为空）的最小删除次数。

「子序列」定义：如果一个字符串可以通过删除原字符串某些字符而不改变原字符顺序得到，那么这个字符串就是原字符串的一个子序列。

「回文」定义：如果一个字符串向后和向前读是一致的，那么这个字符串就是一个回文。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "ababa"
<strong>输出:</strong> 1
<strong>解释:</strong> 字符串本身就是回文序列，只需要删除一次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abb"
<strong>输出:</strong> 2
<strong>解释:</strong> "<strong>a</strong>bb" -> "<strong>bb</strong>" -> "". 
先删除回文子序列 "a"，然后再删除 "bb"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "baabb"
<strong>输出:</strong> 2
<strong>解释:</strong> "<strong>baa</strong>b<strong>b</strong>" -> "b" -> "". 
先删除回文子序列 "baab"，然后再删除 "b"。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = ""
<strong>输出:</strong> 0
</pre>

#### 提示:
* ```0 <= s.length <= 1000```
* ```s``` 仅包含字母 'a'  和 'b'

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def removePalindromeSub(self, s: str) -> int:
        if not s:
            return 0
        if s == s[::-1]:
            return 1
        return 2
```
