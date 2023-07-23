# 1995. 统计特殊四元组
给你一个 **下标从 0 开始** 的整数数组 `nums` ，返回满足下述条件的 **不同** 四元组 `(a, b, c, d)` 的 **数目** ：
* `nums[a] + nums[b] + nums[c] == nums[d]` ，且
* `a < b < c < d`

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,6]
<strong>输出:</strong> 1
<strong>解释:</strong> 满足要求的唯一一个四元组是 (0, 1, 2, 3) 因为 1 + 2 + 3 == 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,3,6,4,5]
<strong>输出:</strong> 0
<strong>解释:</strong> [3,3,6,4,5] 中不存在满足要求的四元组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,1,3,5]
<strong>输出:</strong> 4
<strong>解释:</strong> 满足要求的 4 个四元组如下：
- (0, 1, 2, 3): 1 + 1 + 1 == 3
- (0, 1, 3, 4): 1 + 1 + 3 == 5
- (0, 2, 3, 4): 1 + 1 + 3 == 5
- (1, 2, 3, 4): 1 + 1 + 3 == 5
</pre>

#### 提示:
* `4 <= nums.length <= 50`
* `1 <= nums[i] <= 100`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countQuadruplets(self, nums: List[int]) -> int:
        n = len(nums)
        ret = 0

        for a in range(n):
            for b in range(a + 1, n):
                for c in range(b + 1, n):
                    for d in range(c + 1, n):
                        if nums[a] + nums[b] + nums[c] == nums[d]:
                            ret += 1

        return ret
```
