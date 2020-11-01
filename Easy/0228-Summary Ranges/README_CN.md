# 228. 汇总区间
给定一个无重复元素的有序整数数组 `nums` 。

返回 **恰好覆盖数组中所有数字** 的 **最小有序** 区间范围列表。也就是说，`nums` 的每个元素都恰好被某个区间范围所覆盖，并且不存在属于某个范围但不属于 `nums` 的数字 `x` 。

列表中的每个区间范围 `[a,b]` 应该按如下格式输出：
* `"a->b"` ，如果 `a != b`
* `"a"` ，如果 `a == b`

#### 示例 1:
<pre>
<b>输入:</b> nums = [0,1,2,4,5,7]
<b>输出:</b> ["0->2","4->5","7"]
<b>解释:</b> 区间范围是：
[0,2] --> "0->2"
[4,5] --> "4->5"
[7,7] --> "7"
</pre>

#### 示例 2:
<pre>
<b>输入:</b> nums = [0,2,3,4,6,8,9]
<b>输出:</b> ["0","2->4","6","8->9"]
<b>解释:</b> 区间范围是：
[0,0] --> "0"
[2,4] --> "2->4"
[6,6] --> "6"
[8,9] --> "8->9"
</pre>

#### 示例 3:
<pre>
<b>输入:</b> nums = []
<b>输出:</b> []
</pre>

#### 示例 4:
<pre>
<b>输入:</b> nums = [-1]
<b>输出:</b> ["-1"]
</pre>

#### 示例 5:
<pre>
<b>输入:</b> nums = [0]
<b>输出:</b> ["0"]
</pre>

#### 提示:
* `0 <= nums.length <= 20`
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>
* `nums` 中的所有值都 **互不相同**

## 题解 (Python)

### 1. 题解
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
