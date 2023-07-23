# 686. 重复叠加字符串匹配
给定两个字符串 A 和 B, 寻找重复叠加字符串A的最小次数，使得字符串B成为叠加后的字符串A的子串，如果不存在则返回 -1。

举个例子，A = "abcd"，B = "cdabcdab"。

答案为 3， 因为 A 重复叠加三遍后为 “abcdabcdabcd”，此时 B 是其子串；A 重复叠加两遍后为"abcdabcd"，B 并不是其子串。

#### 注意:
```A``` 与 ```B``` 字符串的长度在1和10000区间范围内。

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def repeatedStringMatch(self, A: str, B: str) -> int:
        r = (len(B) - 1) // len(A) + 1
        if B in A * r:
            return r
        elif B in A * (r + 1):
            return r + 1
        return -1
```
