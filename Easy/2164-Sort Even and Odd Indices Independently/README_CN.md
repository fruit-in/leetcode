# 2164. 对奇偶下标分别排序
给你一个下标从 **0** 开始的整数数组 `nums` 。根据下述规则重排 `nums` 中的值：
1. 按 **非递增** 顺序排列 `nums` **奇数下标** 上的所有值。
    * 举个例子，如果排序前 `nums = [4,1,2,3]` ，对奇数下标的值排序后变为 `[4,3,2,1]` 。奇数下标 `1` 和 `3` 的值按照非递增顺序重排。
2. 按 **非递减** 顺序排列 `nums` **偶数下标** 上的所有值。
    * 举个例子，如果排序前 `nums = [4,1,2,3]` ，对偶数下标的值排序后变为 `[2,1,4,3]` 。偶数下标 `0` 和 `2` 的值按照非递减顺序重排。

返回重排 `nums` 的值之后形成的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,1,2,3]
<strong>输出:</strong> [2,3,4,1]
<strong>解释:</strong>
首先，按非递增顺序重排奇数下标（1 和 3）的值。
所以，nums 从 [4,1,2,3] 变为 [4,3,2,1] 。
然后，按非递减顺序重排偶数下标（0 和 2）的值。
所以，nums 从 [4,1,2,3] 变为 [2,3,4,1] 。
因此，重排之后形成的数组是 [2,3,4,1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,1]
<strong>输出:</strong> [2,1]
<strong>解释:</strong>
由于只有一个奇数下标和一个偶数下标，所以不会发生重排。
形成的结果数组是 [2,1] ，和初始数组一样。
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `1 <= nums[i] <= 100`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def sortEvenOdd(self, nums: List[int]) -> List[int]:
        odds = sorted(nums[i] for i in range(1, len(nums), 2))[::-1]
        evens = sorted(nums[i] for i in range(0, len(nums), 2))

        return [odds[i // 2] if i % 2 == 1 else evens[i // 2] for i in range(len(nums))]
```
