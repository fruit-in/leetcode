# 1569. Number of Ways to Reorder Array to Get Same BST
Given an array `nums` that represents a permutation of integers from `1` to `n`. We are going to construct a binary search tree (BST) by inserting the elements of nums in order into an initially empty BST. Find the number of different ways to reorder `nums` so that the constructed BST is identical to that formed from the original array `nums`.

* For example, given `nums = [2,1,3]`, we will have 2 as the root, 1 as a left child, and 3 as a right child. The array `[2,3,1]` also yields the same BST but `[3,2,1]` yields a different BST.

Return *the number of ways to reorder* `nums` *such that the BST formed is identical to the original BST formed from* `nums`.

Since the answer may be very large, **return it modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/08/12/bb.png)
<pre>
<strong>Input:</strong> nums = [2,1,3]
<strong>Output:</strong> 1
<strong>Explanation:</strong> We can reorder nums to be [2,3,1] which will yield the same BST. There are no other ways to reorder nums which will yield the same BST.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/08/12/ex1.png)
<pre>
<strong>Input:</strong> nums = [3,4,5,1,2]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The following 5 arrays will yield the same BST:
[3,1,2,4,5]
[3,1,4,2,5]
[3,1,4,5,2]
[3,4,1,2,5]
[3,4,1,5,2]
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/08/12/ex4.png)
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no other orderings of nums that will yield the same BST.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `1 <= nums[i] <= nums.length`
* All integers in `nums` are **distinct**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numOfWays(self, nums: List[int]) -> int:
        def numOfWaysIncludingSame(nums: List[int]) -> int:
            if len(nums) <= 2:
                return 1

            left = []
            right = []

            for i in range(1, len(nums)):
                if nums[i] < nums[0]:
                    left.append(nums[i])
                else:
                    right.append(nums[i])

            return math.comb(len(nums) - 1, len(left)) % 1000000007 * numOfWaysIncludingSame(left) % 1000000007 * numOfWaysIncludingSame(right) % 1000000007

        return (numOfWaysIncludingSame(nums) - 1) % 1000000007
```
