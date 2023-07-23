# 844. 比较含退格的字符串
给定 ```S``` 和 ```T``` 两个字符串，当它们分别被输入到空白的文本编辑器后，判断二者是否相等，并返回结果。 ```#``` 代表退格字符。

#### 示例 1:
<pre>
<strong>输入:</strong> S = "ab#c", T = "ad#c"
<strong>输出:</strong> true
<strong>解释:</strong> S 和 T 都会变成 “ac”。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> S = "ab##", T = "c#d#"
<strong>输出:</strong> true
<strong>解释:</strong> S 和 T 都会变成 “”。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> S = "a##c", T = "#a#c"
<strong>输出:</strong> true
<strong>解释:</strong> S 和 T 都会变成 “c”。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> S = "a#c", T = "b"
<strong>输出:</strong> false
<strong>解释:</strong> S 会变成 “c”，但 T 仍然是 “b”。
</pre>

#### 提示:
1. ```1 <= S.length <= 200```
2. ```1 <= T.length <= 200```
3. ```S``` 和 ```T``` 只含有小写字母以及字符 ```'#'```。

## 题解 (Python)

### 1. 栈
```Python3
class Solution:
    def backspaceCompare(self, S: str, T: str) -> bool:
        stack_s = []
        stack_t = []

        for ch in S:
            if ch != '#':
                stack_s.append(ch)
            elif stack_s:
                stack_s.pop()
        for ch in T:
            if ch != '#':
                stack_t.append(ch)
            elif stack_t:
                stack_t.pop()

        return stack_s == stack_t
```

### 2. 双指针
```Python3
class Solution:
    def backspaceCompare(self, S: str, T: str) -> bool:
        i, j = len(S) - 1, len(T) - 1

        while i >= 0 or j >= 0:
            cnt = 0
            while i >= 0 and (S[i] == '#' or cnt > 0):
                cnt += 1 if S[i] == '#' else -1
                i -= 1

            cnt = 0
            while j >= 0 and (T[j] == '#' or cnt > 0):
                cnt += 1 if T[j] == '#' else -1
                j -= 1

            if (i < 0) != (j < 0) or (i + j >= 0 and S[i] != T[j]):
                return False

            i -= 1
            j -= 1

        return True
```
