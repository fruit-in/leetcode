# 1047. Remove All Adjacent Duplicates In String
Given a string ```S``` of lowercase letters, a *duplicate removal* consists of choosing two adjacent and equal letters, and removing them.

We repeatedly make duplicate removals on S until we no longer can.

Return the final string after all such duplicate removals have been made.  It is guaranteed the answer is unique.

#### Example 1:
<pre>
<strong>Input:</strong> "abbaca"
<strong>Output:</strong> "ca"
<strong>Explanation:</strong>
For example, in "abbaca" we could remove "bb" since the letters are adjacent and equal, and this is the only possible move.  The result of this move is that the string is "aaca", of which only "aa" is possible, so the final string is "ca".
</pre>

#### Note:
1. ```1 <= S.length <= 20000```
2. ```S``` consists only of English lowercase letters.

## Solutions (Python)

### 1. Replace
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

### 2. Stack
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
