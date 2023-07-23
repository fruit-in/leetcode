# 1985. Find the Kth Largest Integer in the Array
You are given an array of strings `nums` and an integer `k`. Each string in `nums` represents an integer without leading zeros.

Return *the string that represents the* <code>k<sup>th</sup></code> ***largest integer** in* `nums`.

**Note**: Duplicate numbers should be counted distinctly. For example, if `nums` is `["1","2","2"]`, `"2"` is the first largest integer, `"2"` is the second-largest integer, and `"1"` is the third-largest integer.

#### Example 1:
<pre>
<strong>Input:</strong> nums = ["3","6","7","10"], k = 4
<strong>Output:</strong> "3"
<strong>Explanation:</strong>
The numbers in nums sorted in non-decreasing order are ["3","6","7","10"].
The 4th largest integer in nums is "3".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = ["2","21","12","1"], k = 3
<strong>Output:</strong> "2"
<strong>Explanation:</strong>
The numbers in nums sorted in non-decreasing order are ["1","2","12","21"].
The 3rd largest integer in nums is "2".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = ["0","0"], k = 2
<strong>Output:</strong> "0"
<strong>Explanation:</strong>
The numbers in nums sorted in non-decreasing order are ["0","0"].
The 2nd largest integer in nums is "0".
</pre>

#### Constraints:
* <code>1 <= k <= nums.length <= 10<sup>4</sup></code>
* `1 <= nums[i].length <= 100`
* `nums[i]` consists of only digits.
* `nums[i]` will not have any leading zeros.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def kthLargestNumber(self, nums: List[str], k: int) -> str:
        return sorted(nums, key=lambda num: (len(num), num))[-k]
```
