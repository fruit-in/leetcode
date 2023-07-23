# 1018. Binary Prefix Divisible By 5
Given an array <code>A</code> of <code>0</code>s and <code>1</code>s, consider <code>N_i</code>: the i-th subarray from <code>A[0]</code> to <code>A[i]</code> interpreted as a binary number (from most-significant-bit to least-significant-bit.)

Return a list of booleans <code>answer</code>, where <code>answer[i]</code> is <code>true</code> if and only if <code>N_i</code> is divisible by 5.

#### Example 1:
<pre>
<strong>Input:</strong> [0,1,1]
<strong>Output:</strong> [true,false,false]
<strong>Explanation:</strong>
The input numbers in binary are 0, 01, 011; which are 0, 1, and 3 in base-10.
Only the first number is divisible by 5, so answer[0] is true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,1,1]
<strong>Output:</strong> [false,false,false]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [0,1,1,1,1,1]
<strong>Output:</strong> [true,false,false,false,true,false]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> [1,1,1,0,1]
<strong>Output:</strong> [false,false,false,false,false]
</pre>

#### Note:
1. <code>1 <= A.length <= 30000</code>
2. <code>A[i]</code> is <code>0</code> or <code>1</code>

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def prefixesDivBy5(self, A: List[int]) -> List[bool]:
        n = 0
        for i in range(len(A)):
            n = (2 * n + A[i]) % 5
            A[i] = n == 0
        return A
```
