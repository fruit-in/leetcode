# 2089. 找出数组排序后的目标下标
给你一个下标从 **0** 开始的整数数组 `nums` 以及一个目标元素 `target` 。

**目标下标** 是一个满足 `nums[i] == target` 的下标 `i` 。

将 `nums` 按 **非递减** 顺序排序后，返回由 `nums` 中目标下标组成的列表。如果不存在目标下标，返回一个 **空** 列表。返回的列表必须按 **递增** 顺序排列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,5,2,3], target = 2
<strong>输出:</strong> [1,2]
<strong>解释:</strong> 排序后，nums 变为 [1,2,2,3,5] 。
满足 nums[i] == 2 的下标是 1 和 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,5,2,3], target = 3
<strong>输出:</strong> [3]
<strong>解释:</strong> 排序后，nums 变为 [1,2,2,3,5] 。
满足 nums[i] == 3 的下标是 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,5,2,3], target = 5
<strong>输出:</strong> [4]
<strong>解释:</strong> 排序后，nums 变为 [1,2,2,3,5] 。
满足 nums[i] == 5 的下标是 4 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [1,2,5,2,3], target = 4
<strong>输出:</strong> []
<strong>解释:</strong> nums 中不含值为 4 的元素。
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `1 <= nums[i], target <= 100`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def targetIndices(self, nums: List[int], target: int) -> List[int]:
        nums.sort()

        return list(range(bisect_left(nums, target), bisect_right(nums, target)))
```
