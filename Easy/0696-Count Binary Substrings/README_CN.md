# 696. 计数二进制子串
给定一个字符串 ```s```，计算具有相同数量0和1的非空(连续)子字符串的数量，并且这些子字符串中的所有0和所有1都是组合在一起的。

重复出现的子串要计算它们出现的次数。

#### 示例 1:
<pre>
<strong>输入:</strong> "00110011"
<strong>输出:</strong> 6
<strong>输出:</strong> 有6个子串具有相同数量的连续1和0：“0011”，“01”，“1100”，“10”，“0011” 和 “01”。
请注意，一些重复出现的子串要计算它们出现的次数。
另外，“00110011”不是有效的子串，因为所有的0（和1）没有组合在一起。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "10101"
<strong>输出:</strong> 4
<strong>输出:</strong> 有4个子串：“10”，“01”，“10”，“01”，它们具有相同数量的连续1和0。
</pre>

#### 注意:
* ```s.length``` 在1到50,000之间。
* ```s``` 只包含“0”或“1”字符。

## 题解 (Python)

### 1. 线性扫描
```Python3
class Solution:
    def countBinarySubstrings(self, s: str) -> int:
        prev, curr = 0, 0
        ret = 0

        for i in range(len(s)):
            curr += 1
            if i == len(s) - 1 or s[i] != s[i + 1]:
                ret += min(prev, curr)
                prev, curr = curr, 0

        return ret
```
