# 1985. 找出数组中的第 K 大整数
给你一个字符串数组 `nums` 和一个整数 `k` 。`nums` 中的每个字符串都表示一个不含前导零的整数。

返回 `nums` 中表示第 `k` 大整数的字符串。

**注意**：重复的数字在统计时会视为不同元素考虑。例如，如果 `nums` 是 `["1","2","2"]`，那么 `"2"` 是最大的整数，`"2"` 是第二大的整数，`"1"` 是第三大的整数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = ["3","6","7","10"], k = 4
<strong>输出:</strong> "3"
<strong>解释:</strong>
nums 中的数字按非递减顺序排列为 ["3","6","7","10"]
其中第 4 大整数是 "3"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = ["2","21","12","1"], k = 3
<strong>输出:</strong> "2"
<strong>解释:</strong>
nums 中的数字按非递减顺序排列为 ["1","2","12","21"]
其中第 3 大整数是 "2"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = ["0","0"], k = 2
<strong>输出:</strong> "0"
<strong>解释:</strong>
nums 中的数字按非递减顺序排列为 ["0","0"]
其中第 2 大整数是 "0"
</pre>

#### 提示:
* <code>1 <= k <= nums.length <= 10<sup>4</sup></code>
* `1 <= nums[i].length <= 100`
* `nums[i]` 仅由数字组成
* `nums[i]` 不含任何前导零

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def kthLargestNumber(self, nums: List[str], k: int) -> str:
        return sorted(nums, key=lambda num: (len(num), num))[-k]
```
