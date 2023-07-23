# 1071. 字符串的最大公因子
对于字符串 ```S``` 和 ```T```，只有在 ```S = T + ... + T```（```T``` 与自身连接 1 次或多次）时，我们才认定 “```T``` 能除尽 ```S```”。

返回字符串 ```X```，要求满足 ```X``` 能除尽 ```str1``` 且 ```X``` 能除尽 ```str2```。

#### 示例 1:
<pre>
<strong>输入:</strong> str1 = "ABCABC", str2 = "ABC"
<strong>输出:</strong> "ABC"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> str1 = "ABABAB", str2 = "ABAB"
<strong>输出:</strong> "AB"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> str1 = "LEET", str2 = "CODE"
<strong>输出:</strong> ""
</pre>

#### 提示:
1. ```1 <= str1.length <= 1000```
2. ```1 <= str2.length <= 1000```
3. ```str1[i]``` 和 ```str2[i]``` 为大写英文字母

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def gcdOfStrings(self, str1: str, str2: str) -> str:
        if str1 == str2:
            return str1
        if str1.startswith(str2):
            return self.gcdOfStrings(str1[len(str2):], str2)
        elif str2.startswith(str1):
            return self.gcdOfStrings(str1, str2[len(str1):])
        return ""
```
