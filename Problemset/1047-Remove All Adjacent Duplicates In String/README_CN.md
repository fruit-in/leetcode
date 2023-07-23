# 1047. 删除字符串中的所有相邻重复项
给出由小写字母组成的字符串 ```S```，**重复项删除操作**会选择两个相邻且相同的字母，并删除它们。

在 S 上反复执行重复项删除操作，直到无法继续删除。

在完成所有重复项删除操作后返回最终的字符串。答案保证唯一。

#### 示例:
<pre>
<strong>输入:</strong> "abbaca"
<strong>输出:</strong> "ca"
<strong>解释:</strong>
例如，在 "abbaca" 中，我们可以删除 "bb" 由于两字母相邻且相同，这是此时唯一可以执行删除操作的重复项。之后我们得到字符串 "aaca"，其中又只有 "aa" 可以执行重复项删除操作，所以最后的字符串为 "ca"。
</pre>

#### 提示:
1. ```1 <= S.length <= 20000```
2. ```S``` 仅由小写英文字母组成。

## 题解 (Python)

### 1. 字符串替换
```Python3
class Solution:
    def removeDuplicates(self, S: str) -> str:
        i = 0
        while i + 1 < len(S):
            if S[i] == S[i + 1]:
                S = S.replace(S[i] * 2, '')
                i -= 1 if i > 0 else 0
            else:
                i += 1

        return S
```

### 2. 栈
```Python3
class Solution:
    def removeDuplicates(self, S: str) -> str:
        stack = []
        for ch in S:
            if stack and ch == stack[-1]:
                stack.pop()
            else:
                stack.append(ch)

        return ''.join(stack)
```
