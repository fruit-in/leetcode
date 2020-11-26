# 540. Single Element in a Sorted Array
You are given a sorted array consisting of only integers where every element appears exactly twice, except for one element which appears exactly once. Find this single element that appears only once.

**Follow up:** Your solution should run in O(log n) time and O(1) space.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,2,3,3,4,4,8,8]
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,3,7,7,10,11,11]
<strong>Output:</strong> 10
</pre>

#### Constraints:
* `1 <= nums.length <= 10^5`
* `0 <= nums[i] <= 10^5`

## Solutions (Ruby)

### 1. Binary Search
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def single_non_duplicate(nums)
    l, r = 0, nums.length - 1

    while l <= r
        m = (l + r) / 2

        if m.even?
            if m < nums.length - 1 and nums[m] == nums[m + 1]
                l = m + 1
            elsif m > 0 and nums[m] == nums[m - 1]
                r = m - 1
            else
                return nums[m]
            end
        else
            if nums[m] == nums[m + 1]
                r = m - 1
            elsif nums[m] == nums[m - 1]
                l = m + 1
            else
                return nums[m]
            end
        end
    end
end
```
