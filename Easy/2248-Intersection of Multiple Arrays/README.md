# 2248. Intersection of Multiple Arrays
Given a 2D integer array `nums` where `nums[i]` is a non-empty array of **distinct** positive integers, return *the list of integers that are present in **each array** of* `nums` *sorted in **ascending order***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [[3,1,2,4,5],[1,2,3,4],[3,4,5,6]]
<strong>Output:</strong> [3,4]
<strong>Explanation:</strong>
The only integers present in each of nums[0] = [3,1,2,4,5], nums[1] = [1,2,3,4], and nums[2] = [3,4,5,6] are 3 and 4, so we return [3,4].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [[1,2,3],[4,5,6]]
<strong>Output:</strong> []
<strong>Explanation:</strong>
There does not exist any integer present both in nums[0] and nums[1], so we return an empty list [].
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `1 <= sum(nums[i].length) <= 1000`
* `1 <= nums[i][j] <= 1000`
* All the values of `nums[i]` are **unique**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def intersection(self, nums: List[List[int]]) -> List[int]:
        count = {}

        for array in nums:
            for num in array:
                if num not in count:
                    count[num] = 0
                count[num] += 1

        return sorted(k for k, v in count.items() if v == len(nums))
```
