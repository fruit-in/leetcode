# 2164. Sort Even and Odd Indices Independently
You are given a **0-indexed** integer array `nums`. Rearrange the values of `nums` according to the following rules:
1. Sort the values at **odd indices** of `nums` in **non-increasing** order.
    * For example, if `nums = [4,1,2,3]` before this step, it becomes `[4,3,2,1]` after. The values at odd indices `1` and `3` are sorted in non-increasing order.
2. Sort the values at **even indices** of `nums` in **non-decreasing** order.
    * For example, if `nums = [4,1,2,3]` before this step, it becomes `[2,1,4,3]` after. The values at even indices `0` and `2` are sorted in non-decreasing order.

Return *the array formed after rearranging the values of* `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,1,2,3]
<strong>Output:</strong> [2,3,4,1]
<strong>Explanation:</strong>
First, we sort the values present at odd indices (1 and 3) in non-increasing order.
So, nums changes from [4,1,2,3] to [4,3,2,1].
Next, we sort the values present at even indices (0 and 2) in non-decreasing order.
So, nums changes from [4,1,2,3] to [2,3,4,1].
Thus, the array formed after rearranging the values is [2,3,4,1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,1]
<strong>Output:</strong> [2,1]
<strong>Explanation:</strong>
Since there is exactly one odd index and one even index, no rearrangement of values takes place.
The resultant array formed is [2,1], which is the same as the initial array.
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `1 <= nums[i] <= 100`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def sortEvenOdd(self, nums: List[int]) -> List[int]:
        odds = sorted(nums[i] for i in range(1, len(nums), 2))[::-1]
        evens = sorted(nums[i] for i in range(0, len(nums), 2))

        return [odds[i // 2] if i % 2 == 1 else evens[i // 2] for i in range(len(nums))]
```
