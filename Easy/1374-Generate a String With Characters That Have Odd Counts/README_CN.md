# 1374. 生成每种字符都是奇数个的字符串
给你一个整数 ```n```，请你返回一个含 *```n```* 个字符的字符串，其中每种字符在该字符串中都恰好出现 **奇数次** 。

返回的字符串必须只含小写英文字母。如果存在多个满足题目要求的字符串，则返回其中任意一个即可。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> "pppz"
<strong>解释:</strong> "pppz" 是一个满足题目要求的字符串，因为 'p' 出现 3 次，且 'z' 出现 1 次。当然，还有很多其他字符串也满足题目要求，比如："ohhh" 和 "love"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> "xy"
<strong>解释:</strong> "xy" 是一个满足题目要求的字符串，因为 'x' 和 'y' 各出现 1 次。当然，还有很多其他字符串也满足题目要求，比如："ag" 和 "ur"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 7
<strong>输出:</strong> "holasss"
</pre>

#### 提示:
* ```1 <= n <= 500```

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def generateTheString(self, n: int) -> str:
        return 'a' * (n - 1 + n % 2) + 'b' * ((n + 1) % 2)
```
