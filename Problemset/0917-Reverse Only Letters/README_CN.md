# 917. 仅仅反转字母
给定一个字符串 ```S```，返回 “反转后的” 字符串，其中不是字母的字符都保留在原地，而所有字母的位置发生反转。

#### 示例 1:
<pre>
<strong>输入:</strong> "ab-cd"
<strong>输出:</strong> "dc-ba"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "a-bC-dEf-ghIj"
<strong>输出:</strong> "j-Ih-gfE-dCba"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "Test1ng-Leet=code-Q!"
<strong>输出:</strong> "Qedo1ct-eeLg=ntse-T!"
</pre>

#### 提示:
1. ```S.length <= 100```
2. ```33 <= S[i].ASCIIcode <= 122```
3. ```S``` 中不包含 ```\``` or ```"```

## 题解 (Python)

### 1. 双指针
```Python
class Solution:
    def reverseOnlyLetters(self, S: str) -> str:
        S = list(S)
        i, j = 0, len(S) - 1

        while i < j:
            if S[i].isalpha() and S[j].isalpha():
                S[i], S[j] = S[j], S[i]
                i += 1
                j -= 1
            elif not S[i].isalpha():
                i += 1
            elif not S[j].isalpha():
                j -= 1

        return ''.join(S)
```

### 2. 栈
```Python
class Solution:
    def reverseOnlyLetters(self, S: str) -> str:
        stack = []
        ret = ""

        for ch in S:
            if ch.isalpha():
                stack.append(ch)

        for ch in S:
            if ch.isalpha():
                ret += stack.pop()
            else:
                ret += ch

        return ret
```
