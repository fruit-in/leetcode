# 1003. 检查替换后的词是否有效
给定有效字符串 ```"abc"```。

对于任何有效的字符串 ```V```，我们可以将 ```V``` 分成两个部分 ```X``` 和 ```Y```，使得 ```X + Y```（```X``` 与 ```Y``` 连接）等于 ```V```。（```X``` 或 ```Y``` 可以为空。）那么，```X + "abc" + Y``` 也同样是有效的。

例如，如果 ```S = "abc"```，则有效字符串的示例是：```"abc"```，```"aabcbc"```，```"abcabc"```，```"abcabcababcc"```。**无效**字符串的示例是：```"abccba"```，```"ab"```，```"cababc"```，```"bac"```。

如果给定字符串 ```S``` 有效，则返回 ```true```；否则，返回 ```false```。

#### 示例 1:
<pre>
<strong>输入:</strong> "aabcbc"
<strong>输出:</strong> true
<strong>解释:</strong>
从有效字符串 "abc" 开始。
然后我们可以在 "a" 和 "bc" 之间插入另一个 "abc"，产生 "a" + "abc" + "bc"，即 "aabcbc"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "abcabcababcc"
<strong>输出:</strong> true
<strong>解释:</strong>
"abcabcabc" 是有效的，它可以视作在原串后连续插入 "abc"。
然后我们可以在最后一个字母之前插入 "abc"，产生 "abcabcab" + "abc" + "c"，即 "abcabcababcc"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "abccba"
<strong>输出:</strong> false
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> "cababc"
<strong>输出:</strong> false
</pre>

#### 提示:
1. ```1 <= S.length <= 20000```
2. ```S[i]``` 为 ```'a'```、```'b'```、或 ```'c'```

## 题解 (Python)

### 1. 字符串替换
```Python
class Solution:
    def isValid(self, S: str) -> bool:
        while 'abc' in S:
            S = S.replace('abc', '')

        return not S
```
