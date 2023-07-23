# 1509. Minimum Difference Between Largest and Smallest Value in Three Moves
Given an array `nums`, you are allowed to choose one element of `nums` and change it by any value in one move.

Return the minimum difference between the largest and smallest value of `nums` after perfoming at most 3 moves.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [5,3,2,4]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Change the array [5,3,2,4] to [2,2,2,2].
The difference between the maximum and minimum is 2-2 = 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,5,0,10,14]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Change the array [1,5,0,10,14] to [1,1,0,1,1].
The difference between the maximum and minimum is 1-0 = 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [6,6,0,1,1,4,6]
<strong>Output:</strong> 2
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> nums = [1,5,6,14,15]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= nums.length <= 10^5`
* `-10^9 <= nums[i] <= 10^9`

## Solutions (Ruby)

### 1. Greedy
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def min_difference(nums)
  return 0 if nums.length < 5

  maxs = nums.max(4)
  mins = nums.min(4)

  [maxs[0] - mins[3], maxs[1] - mins[2], maxs[2] - mins[1], maxs[3] - mins[0]].min
end
```
