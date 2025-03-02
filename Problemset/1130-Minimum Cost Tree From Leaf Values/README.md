# 1130. Minimum Cost Tree From Leaf Values
Given an array `arr` of positive integers, consider all binary trees such that:
* Each node has either `0` or `2` children;
* The values of `arr` correspond to the values of each **leaf** in an in-order traversal of the tree.
* The value of each non-leaf node is equal to the product of the largest leaf value in its left and right subtree, respectively.

Among all possible binary trees considered, return *the smallest possible sum of the values of each non-leaf node*. It is guaranteed this sum fits into a **32-bit** integer.

A node is a **leaf** if and only if it has zero children.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/08/10/tree1.jpg)
<pre>
<strong>Input:</strong> arr = [6,2,4]
<strong>Output:</strong> 32
<strong>Explanation:</strong> There are two possible trees shown.
The first has a non-leaf node sum 36, and the second has non-leaf node sum 32.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/08/10/tree2.jpg)
<pre>
<strong>Input:</strong> arr = [4,11]
<strong>Output:</strong> 44
</pre>

#### Constraints:
* `2 <= arr.length <= 40`
* `1 <= arr[i] <= 15`
* It is guaranteed that the answer fits into a **32-bit** signed integer (i.e., it is less than 2<sup>31</sup>).

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def mctFromLeafValues(self, arr: List[int]) -> int:
        @cache
        def mctFromSub(i: int, j: int) -> (int, int):
            if i + 1 == j:
                return (0, arr[i])

            treesum, treemax = 1 << 31, 0

            for k in range(i + 1, j):
                leftsum, leftmax = mctFromSub(i, k)
                rightsum, rightmax = mctFromSub(k, j)
                treesum, treemax = min(
                    treesum, leftsum + rightsum + leftmax * rightmax), max(leftmax, rightmax)

            return (treesum, treemax)

        return mctFromSub(0, len(arr))[0]
```
