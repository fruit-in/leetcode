# 2248. 多个数组求交集
给你一个二维整数数组 `nums` ，其中 `nums[i]` 是由 **不同** 正整数组成的一个非空数组，按 **升序排列** 返回一个数组，数组中的每个元素在 `nums` **所有数组** 中都出现过。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [[3,1,2,4,5],[1,2,3,4],[3,4,5,6]]
<strong>输出:</strong> [3,4]
<strong>解释:</strong>
nums[0] = [3,1,2,4,5]，nums[1] = [1,2,3,4]，nums[2] = [3,4,5,6]，在 nums 中每个数组中都出现的数字是 3 和 4 ，所以返回 [3,4] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [[1,2,3],[4,5,6]]
<strong>输出:</strong> []
<strong>解释:</strong>
不存在同时出现在 nums[0] 和 nums[1] 的整数，所以返回一个空列表 [] 。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `1 <= sum(nums[i].length) <= 1000`
* `1 <= nums[i][j] <= 1000`
* `nums[i]` 中的所有值 **互不相同**

## 题解 (Python)

### 1. 题解
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
