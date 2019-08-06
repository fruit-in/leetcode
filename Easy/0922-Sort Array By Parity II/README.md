# 922. Sort Array By Parity II
Given an array <code>A</code> of non-negative integers, half of the integers in A are odd, and half of the integers are even.

Sort the array so that whenever <code>A[i]</code> is odd, <code>i</code> is odd; and whenever <code>A[i]</code> is even, <code>i</code> is even.

You may return any answer array that satisfies this condition.

#### Example 1:
<pre>
<strong>Input:</strong> [4,2,5,7]
<strong>Output:</strong> [4,5,2,7]
<strong>Explanation:</strong> [4,7,2,5], [2,5,4,7], [2,7,4,5] would also have been accepted.
</pre>

#### Note:
1. <code>2 <= A.length <= 20000</code>
2. <code>A.length % 2 == 0</code>
3. <code>0 <= A[i] <= 1000</code>

## Solutions (Python)

### 1. One Pass
```Python3
class Solution:
    def sortArrayByParityII(self, A: List[int]) -> List[int]:
        e, o = 0, 1
        result = [None] * len(A)
        for n in A:
            if n % 2 == 0:
                result[e] = n
                e += 2
            else:
                result[o] = n
                o += 2
        return result
```

### 2. Two Points
```Python3
class Solution:
    def sortArrayByParityII(self, A: List[int]) -> List[int]:
        j = 1
        for i in range(0, len(A), 2):
            if A[i] % 2:
                while A[j] % 2:
                    j += 2
                A[i], A[j] = A[j], A[i]
        return A
```
