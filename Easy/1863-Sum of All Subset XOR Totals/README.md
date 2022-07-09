# 1863. Sum of All Subset XOR Totals
The **XOR total** of an array is defined as the bitwise `XOR` of **all its elements**, or `0` if the array is **empty**.
* For example, the **XOR total** of the array `[2,5,6]` is `2 XOR 5 XOR 6 = 1`.

Given an array `nums`, return *the **sum** of all **XOR totals** for every **subset** of* `nums`.

**Note:** Subsets with the **same** elements should be counted **multiple** times.

An array `a` is a **subset** of an array `b` if `a` can be obtained from `b` by deleting some (possibly zero) elements of `b`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The 4 subsets of [1,3] are:
- The empty subset has an XOR total of 0.
- [1] has an XOR total of 1.
- [3] has an XOR total of 3.
- [1,3] has an XOR total of 1 XOR 3 = 2.
0 + 1 + 3 + 2 = 6
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,1,6]
<strong>Output:</strong> 28
<strong>Explanation:</strong> The 8 subsets of [5,1,6] are:
- The empty subset has an XOR total of 0.
- [5] has an XOR total of 5.
- [1] has an XOR total of 1.
- [6] has an XOR total of 6.
- [5,1] has an XOR total of 5 XOR 1 = 4.
- [5,6] has an XOR total of 5 XOR 6 = 3.
- [1,6] has an XOR total of 1 XOR 6 = 7.
- [5,1,6] has an XOR total of 5 XOR 1 XOR 6 = 2.
0 + 5 + 1 + 6 + 4 + 3 + 7 + 2 = 28
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,4,5,6,7,8]
<strong>Output:</strong> 480
<strong>Explanation:</strong> The sum of all XOR totals for every subset is 480.
</pre>

#### Constraints:
* `1 <= nums.length <= 12`
* `1 <= nums[i] <= 20`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def subsetXORSum(self, nums: List[int]) -> int:
        ret = 0

        for n in range(1, len(nums) + 1):
            for subset in combinations(nums, n):
                ret += reduce(lambda x, y: x ^ y, subset)

        return ret
```
