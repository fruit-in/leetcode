# 378. 有序矩阵中第 K 小的元素
给你一个 `n x n` 矩阵 `matrix` ，其中每行和每列元素均按升序排序，找到矩阵中第 `k` 小的元素。
请注意，它是 **排序后** 的第 `k` 小元素，而不是第 `k` 个 **不同** 的元素。

你必须找到一个内存复杂度优于 <code>O(n<sup>2</sup>)</code> 的解决方案。

#### 示例 1:
<pre>
<strong>输入:</strong> matrix = [[1,5,9],[10,11,13],[12,13,15]], k = 8
<strong>输出:</strong> 13
<strong>解释:</strong> 矩阵中的元素为 [1,5,9,10,11,12,13,13,15]，第 8 小元素是 13
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> matrix = [[-5]], k = 1
<strong>输出:</strong> -5
</pre>

#### 提示:
* `n == matrix.length`
* `n == matrix[i].length`
* `1 <= n <= 300`
* <code>-109 <= matrix[i][j] <= 10<sup>9</sup></code>
* 题目数据 **保证** `matrix` 中的所有行和列都按 **非递减顺序** 排列
* <code>1 <= k <= n<sup>2</sup></code>

#### 进阶:
* 你能否用一个恒定的内存(即 `O(1)` 内存复杂度)来解决这个问题?
* 你能在 `O(n)` 的时间复杂度下解决这个问题吗?这个方法对于面试来说可能太超前了，但是你会发现阅读这篇文章（ [this paper](http://www.cse.yorku.ca/~andy/pubs/X+Y.pdf) ）很有趣。

## 题解 (Python)

### 1. 题解
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
