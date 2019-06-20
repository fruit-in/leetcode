# 969. Pancake Sorting
Given an array <code>A</code>, we can perform a *pancake flip*: We choose some positive integer <code>k <= A.length</code>, then reverse the order of the first **k** elements of <code>A</code>. We want to perform zero or more pancake flips (doing them one after another in succession) to sort the array <code>A</code>.

Return the k-values corresponding to a sequence of pancake flips that sort <code>A</code>.  Any valid answer that sorts the array within <code>10 * A.length</code> flips will be judged as correct.

#### Example 1:
<pre>
<strong>Input:</strong> [3,2,4,1]
<strong>Output:</strong> [4,2,4,3]
<strong>Explanation:</strong>
We perform 4 pancake flips, with k values 4, 2, 4, and 3.
Starting state: A = [3, 2, 4, 1]
After 1st flip (k=4): A = [1, 4, 2, 3]
After 2nd flip (k=2): A = [4, 1, 2, 3]
After 3rd flip (k=4): A = [3, 2, 1, 4]
After 4th flip (k=3): A = [1, 2, 3, 4], which is sorted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2,3]
<strong>Output:</strong> []
<strong>Explanation:</strong> The input is already sorted, so there is no need to flip anything.
Note that other answers, such as [3, 3], would also be accepted.
</pre>

#### Note:
1. <code>1 <= A.length <= 100</code>
2. <code>A[i]</code> is a permutation of <code>[1, 2, ..., A.length]</code>

## Solutions

### 1. Move to the First Position, then Move to the Correct Position (Python3)
```Python3
class Solution:
    def pancakeSort(self, A: List[int]) -> List[int]:
        finalA = sorted(A)
        ks = []
        n = len(A)
        for n in range(len(A), 0, -1):
            if A == finalA:
                break
            index = A.index(n)
            if index == n - 1:
                continue
            if index != 0:
                A[:index + 1] = A[index::-1]
                ks.append(index + 1)
            A[:n] = A[n - 1::-1]
            ks.append(n)
        return ks
```
