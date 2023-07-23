# 821. 字符的最短距离
给定一个字符串 ```S``` 和一个字符 ```C```。返回一个代表字符串 ```S``` 中每个字符到字符串 ```S``` 中的字符 ```C``` 的最短距离的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> S = "loveleetcode", C = 'e'
<strong>输出:</strong> [3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
</pre>

#### 说明:
1. 字符串 ```S``` 的长度范围为 ```[1, 10000]```。
2. ```C``` 是一个单字符，且保证是字符串 ```S``` 里的字符。
3. ```S``` 和 ```C``` 中的所有字母均为小写字母。

## 题解 (Python)

### 1. 暴力法
```Python3
class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        ret = []

        for i in range(len(S)):
            shortest = len(S)

            for j in range(len(S)):
                if S[j] == C:
                    shortest = min(shortest, abs(i - j))

            ret.append(shortest)

        return ret
```

### 2. 查找两边
```Python3
class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        ret = []

        for i in range(len(S)):
            for j in range(len(S)):
                l_char = S[i - j] if i - j >= 0 else ''
                r_char = S[i + j] if i + j < len(S) else ''

                if l_char == C or r_char == C:
                    ret.append(j)
                    break

        return ret
```

### 3. 最小数组
```Python3
class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        prev = len(S)
        left = []

        for ch in S:
            prev += 1
            if ch == C:
                prev = 0
            left.append(prev)

        prev = len(S)
        right = []

        for ch in S[::-1]:
            prev += 1
            if ch == C:
                prev = 0
            right.append(prev)
        right.reverse()

        return [min(left[i], right[i]) for i in range(len(S))]
```

### 4. 三指针
```Python3
class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        prev = -len(S)
        next = S.find(C)
        ret = []

        for i in range(len(S)):
            ret.append(min(i - prev, next - i))

            if i == next:
                prev = next
                try:
                    next = S.index(C, next + 1)
                except:
                    next = 2 * len(S)

        return ret
```
