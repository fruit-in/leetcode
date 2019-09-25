# 171. Excel表列序号
给定一个Excel表格中的列名称，返回其相应的列序号。

例如，
```
    A -> 1
    B -> 2
    C -> 3
    ...
    Z -> 26
    AA -> 27
    AB -> 28
    ...
```

#### 示例 1:
<pre>
<strong>输入:</strong> "A"
<strong>输出:</strong> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "AB"
<strong>输出:</strong> 28
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "ZY"
<strong>输出:</strong> 701
</pre>

#### 致谢:
特别感谢 [@ts](https://leetcode.com/discuss/user/ts) 添加此问题并创建所有测试用例。

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def titleToNumber(self, s: str) -> int:
        ret = 0
        for c in s:
            ret *= 26
            ret += ord(c) - 64
        return ret
```
