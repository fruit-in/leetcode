# 942. DI String Match
Given a string <code>S</code> that **only** contains "I" (increase) or "D" (decrease), let <code>N = S.length</code>.

Return **any** permutation <code>A</code> of <code>[0, 1, ..., N]</code> such that for all <code>i = 0, ..., N-1</code>:
* If <code>S[i] == "I"</code>, then <code>A[i] < A[i+1]</code>
* If <code>S[i] == "D"</code>, then <code>A[i] > A[i+1]</code>

#### Example 1:
<pre>
<strong>Input:</strong> "IDID"
<strong>Output:</strong> [0,4,1,3,2]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "III"
<strong>Output:</strong> [0,1,2,3]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "DDI"
<strong>Output:</strong> [3,2,0,1]
</pre>

#### Note:
1. <code>1 <= S.length <= 10000</code>
2. <code>S</code> only contains characters <code>"I"</code> or <code>"D"</code>.

## Solutions

### 1. Solution (Python3)
```Python3
class Solution:
    def diStringMatch(self, S: str) -> List[int]:
        b, e = 0, len(S)
        A = []
        for di in S:
            if di == 'I':
                A.append(b)
                b += 1
            elif di == 'D':
                A.append(e)
                e -= 1
        A.append(b)
        return A
```
