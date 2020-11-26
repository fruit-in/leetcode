# 540. 有序数组中的单一元素
给定一个只包含整数的有序数组，每个元素都会出现两次，唯有一个数只会出现一次，找出这个数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,2,3,3,4,4,8,8]
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,3,7,7,10,11,11]
<strong>输出:</strong> 10
</pre>

**注意:** 您的方案应该在 O(log n)时间复杂度和 O(1)空间复杂度中运行。

## 题解 (Ruby)

### 1. 二分查找
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
