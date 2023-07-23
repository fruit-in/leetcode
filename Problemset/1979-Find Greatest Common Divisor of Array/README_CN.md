# 1979. 找出数组的最大公约数
给你一个整数数组 `nums` ，返回数组中最大数和最小数的 **最大公约数** 。

两个数的 **最大公约数** 是能够被两个数整除的最大正整数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,5,6,9,10]
<strong>输出:</strong> 2
<strong>解释:</strong>
nums 中最小的数是 2
nums 中最大的数是 10
2 和 10 的最大公约数是 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [7,5,6,8,3]
<strong>输出:</strong> 1
<strong>解释:</strong>
nums 中最小的数是 3
nums 中最大的数是 8
3 和 8 的最大公约数是 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,3]
<strong>输出:</strong> 3
<strong>解释:</strong>
nums 中最小的数是 3
nums 中最大的数是 3
3 和 3 的最大公约数是 3
</pre>

#### 提示:
* `2 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findGCD(self, nums: List[int]) -> int:
        return gcd(min(nums), max(nums))
```

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def find_gcd(nums)
  nums.min.gcd(nums.max)
end
```
