# 378. Kth Smallest Element in a Sorted Matrix
Given an `n x n` `matrix` where each of the rows and columns is sorted in ascending order, return *the* <code>k<sup>th</sup></code> *smallest element in the matrix*.

Note that it is the <code>k<sup>th</sup></code> smallest element **in the sorted order**, not the <code>k<sup>th</sup></code> **distinct** element.

You must find a solution with a memory complexity better than <code>O(n<sup>2</sup>)</code>.

#### Example 1:
<pre>
<strong>Input:</strong> matrix = [[1,5,9],[10,11,13],[12,13,15]], k = 8
<strong>Output:</strong> 13
<strong>Explanation:</strong> The elements in the matrix are [1,5,9,10,11,12,13,13,15], and the 8th smallest number is 13
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> matrix = [[-5]], k = 1
<strong>Output:</strong> -5
</pre>

#### Constraints:
* `n == matrix.length == matrix[i].length`
* `1 <= n <= 300`
* <code>-109 <= matrix[i][j] <= 10<sup>9</sup></code>
* All the rows and columns of `matrix` are **guaranteed** to be sorted in **non-decreasing order**.
* <code>1 <= k <= n<sup>2</sup></code>

#### Follow up:
* Could you solve the problem with a constant memory (i.e., `O(1)` memory complexity)?
* Could you solve the problem in `O(n)` time complexity? The solution may be too advanced for an interview but you may find reading [this paper](http://www.cse.yorku.ca/~andy/pubs/X+Y.pdf) fun.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def kthSmallest(self, matrix: List[List[int]], k: int) -> int:
        n = len(matrix)
        r, c = n - 1, 0
        countle = 0
        countge = 0

        while countle < k or countge < n * n - k + 1:
            countle = sum(bisect.bisect(
                matrix[i], matrix[r][c]) for i in range(n))
            countge = sum(
                n - bisect.bisect(matrix[i], matrix[r][c] - 1) for i in range(n))
            if countle < k:
                c += 1
            if countge < n * n - k + 1:
                r -= 1

        return matrix[r][c]
```
