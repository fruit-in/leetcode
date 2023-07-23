# 228. Summary Ranges
You are given a **sorted unique** integer array `nums`.

Return *the **smallest sorted** list of ranges that **cover all the numbers in the array exactly***. That is, each element of `nums` is covered by exactly one of the ranges, and there is no integer `x` such that `x` is in one of the ranges but not in `nums`.

Each range `[a,b]` in the list should be output as:
* `"a->b"` if `a != b`
* `"a"` if `a == b`

#### Example 1:
<pre>
<b>Input:</b> nums = [0,1,2,4,5,7]
<b>Output:</b> ["0->2","4->5","7"]
<b>Explanation:</b> The ranges are:
[0,2] --> "0->2"
[4,5] --> "4->5"
[7,7] --> "7"
</pre>

#### Example 2:
<pre>
<b>Input:</b> nums = [0,2,3,4,6,8,9]
<b>Output:</b> ["0","2->4","6","8->9"]
<b>Explanation:</b> The ranges are:
[0,0] --> "0"
[2,4] --> "2->4"
[6,6] --> "6"
[8,9] --> "8->9"
</pre>

#### Example 3:
<pre>
<b>Input:</b> nums = []
<b>Output:</b> []
</pre>

#### Example 4:
<pre>
<b>Input:</b> nums = [-1]
<b>Output:</b> ["-1"]
</pre>

#### Example 5:
<pre>
<b>Input:</b> nums = [0]
<b>Output:</b> ["0"]
</pre>

#### Constraints:
* `0 <= nums.length <= 20`
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>
* All the values of `nums` are **unique**.
* `nums` is sorted in ascending order.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def summaryRanges(self, nums: List[int]) -> List[str]:
        i = 0
        ret = []

        for j in range(len(nums)):
            if j == len(nums) - 1 or nums[j] + 1 != nums[j + 1]:
                if i == j:
                    ret.append(str(nums[i]))
                else:
                    ret.append(str(nums[i]) + "->" + str(nums[j]))
                i = j + 1

        return ret
```
