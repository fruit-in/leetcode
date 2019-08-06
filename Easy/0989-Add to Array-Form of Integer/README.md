# 989. Add to Array-Form of Integer
For a non-negative integer <code>X</code>, the *array-form of <code>X</code>* is an array of its digits in left to right order.  For example, if <code>X = 1231</code>, then the array form is <code>[1,2,3,1]</code>.

Given the array-form <code>A</code> of a non-negative integer <code>X</code>, return the array-form of the integer <code>X+K</code>.

#### Example 1:
<pre>
<strong>Input:</strong> A = [1,2,0,0], K = 34
<strong>Output:</strong> [1,2,3,4]
<strong>Explanation:</strong> 1200 + 34 = 1234
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> A = [2,7,4], K = 181
<strong>Output:</strong> [4,5,5]
<strong>Explanation:</strong> 274 + 181 = 455
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> A = [2,1,5], K = 806
<strong>Output:</strong> [1,0,2,1]
<strong>Explanation:</strong> 215 + 806 = 1021
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> A = [9,9,9,9,9,9,9,9,9,9], K = 1
<strong>Output:</strong> [1,0,0,0,0,0,0,0,0,0,0]
<strong>Explanation:</strong> 9999999999 + 1 = 10000000000
</pre>

#### Note:
1. <code>1 <= A.length <= 10000</code>
2. <code>0 <= A[i] <= 9</code>
3. <code>0 <= K <= 10000</code>
4. If <code>A.length > 1</code>, then <code>A[0] != 0</code>

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def addToArrayForm(self, A: List[int], K: int) -> List[int]:
        A[-1] += K
        i = -1
        while A[i] > 9:
            if len(A) > -i:
                A[i - 1] += A[i] // 10
            else:
                A = [A[i] // 10] + A
            A[i] %= 10
            i -= 1
        return A
```
