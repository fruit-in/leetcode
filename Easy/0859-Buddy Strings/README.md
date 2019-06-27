# 859. Buddy Strings
Given two strings <code>A</code> and <code>B</code> of lowercase letters, return <code>true</code> if and only if we can swap two letters in <code>A</code> so that the result equals <code>B</code>.

#### Example 1:
<pre>
<strong>Input:</strong> A = "ab", B = "ba"
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> A = "ab", B = "ab"
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> A = "aa", B = "aa"
<strong>Output:</strong> true
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> A = "aaaaaaabc", B = "aaaaaaacb"
<strong>Output:</strong> true
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> A = "", B = "aa"
<strong>Output:</strong> false
</pre>

#### Note:
1. <code>0 <= A.length <= 20000</code>
2. <code>0 <= B.length <= 20000</code>
3. <code>A</code> and <code>B</code> consist only of lowercase letters.

## Solutions

### 1. Solution (Python3)
```Python3
class Solution:
    def buddyStrings(self, A: str, B: str) -> bool:
        if len(A) != len(B):
            return False
        if A == B and len(set(A)) != len(A):
            return True
        a, b = '', ''
        for k, v in enumerate(A):
            if v != B[k]:
                a += v
                b += B[k]
            if len(a) > 2:
                return False
        return len(a) == 2 and a == b[::-1]
```
