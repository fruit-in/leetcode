# 917. Reverse Only Letters
Given a string ```S```, return the "reversed" string where all characters that are not a letter stay in the same place, and all letters reverse their positions.

#### Example 1:
<pre>
<strong>Input:</strong> "ab-cd"
<strong>Output:</strong> "dc-ba"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "a-bC-dEf-ghIj"
<strong>Output:</strong> "j-Ih-gfE-dCba"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "Test1ng-Leet=code-Q!"
<strong>Output:</strong> "Qedo1ct-eeLg=ntse-T!"
</pre>

#### Note:
1. ```S.length <= 100```
2. ```33 <= S[i].ASCIIcode <= 122```
3. ```S``` doesn't contain ```\``` or ```"```

## Solutions (Python)

### 1. Two Pointers
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

### 2. Stack
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
