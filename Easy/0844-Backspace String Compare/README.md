# 844. Backspace String Compare
Given two strings ```S``` and ```T```, return if they are equal when both are typed into empty text editors. ```#``` means a backspace character.

#### Example 1:
<pre>
<strong>Input:</strong> S = "ab#c", T = "ad#c"
<strong>Output:</strong> true
<strong>Explanation:</strong> Both S and T become "ac".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> S = "ab##", T = "c#d#"
<strong>Output:</strong> true
<strong>Explanation:</strong> Both S and T become "".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> S = "a##c", T = "#a#c"
<strong>Output:</strong> true
<strong>Explanation:</strong> Both S and T become "c".
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> S = "a#c", T = "b"
<strong>Output:</strong> false
<strong>Explanation:</strong> S becomes "c" while T becomes "b".
</pre>

#### Note:
1. ```1 <= S.length <= 200```
2. ```1 <= T.length <= 200```
3. ```S``` and ```T``` only contain lowercase letters and ```'#'``` characters.

#### Follow up:
* Can you solve it in ```O(N)``` time and ```O(1)``` space?

## Solutions (Python)

### 1. Stack
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

### 2. Two Pointers
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
