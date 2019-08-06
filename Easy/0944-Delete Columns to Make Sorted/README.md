# 944. Delete Columns to Make Sorted
We are given an array <code>A</code> of <code>N</code> lowercase letter strings, all of the same length.

Now, we may choose any set of deletion indices, and for each string, we delete all the characters in those indices.

For example, if we have an array <code>A = ["abcdef","uvwxyz"]</code> and deletion indices <code>{0, 2, 3}</code>, then the final array after deletions is <code>["bef", "vyz"]</code>, and the remaining columns of <code>A</code> are <code>["b","v"]</code>, <code>["e","y"]</code>, and <code>["f","z"]</code>.  (Formally, the <code>c</code>-th column is <code>[A[0][c], A[1][c], ..., A[A.length-1][c]]</code>.)

Suppose we chose a set of deletion indices <code>D</code> such that after deletions, each remaining column in A is in **non-decreasing** sorted order.

Return the minimum possible value of <code>D.length</code>.

#### Example 1:
<pre>
<strong>Input:</strong> ["cba","daf","ghi"]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
After choosing D = {1}, each column ["c","d","g"] and ["a","f","i"] are in non-decreasing sorted order.
If we chose D = {}, then a column ["b","a","h"] would not be in non-decreasing sorted order.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ["a","b"]
<strong>Output:</strong> 0
<strong>Explanation:</strong> D = {}
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> ["zyx","wvu","tsr"]
<strong>Output:</strong> 3
<strong>Explanation:</strong> D = {0, 1, 2}
</pre>

#### Note:
1. <code>1 <= A.length <= 100</code>
2. <code>1 <= A[i].length <= 1000</code>

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def minDeletionSize(self, A: List[str]) -> int:
        return len(list(filter(lambda B : B != sorted(B), map(list, zip(*A)))))
```
