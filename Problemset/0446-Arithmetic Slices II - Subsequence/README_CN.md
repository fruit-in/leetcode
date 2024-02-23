# 446. 等差数列划分 II - 子序列
给你一个整数数组 `nums` ，返回 `nums` 中所有 **等差子序列** 的数目。

如果一个序列中 **至少有三个元素** ，并且任意两个相邻元素之差相同，则称该序列为等差序列。

* 例如，`[1, 3, 5, 7, 9]`、`[7, 7, 7, 7]` 和 `[3, -1, -5, -9]` 都是等差序列。
* 再例如，`[1, 1, 2, 5, 7]` 不是等差序列。

数组中的子序列是从数组中删除一些元素（也可能不删除）得到的一个序列。

* 例如，`[2,5,10]` 是 `[1,2,1,2,4,1,5,10]` 的一个子序列。

题目数据保证答案是一个 **32-bit** 整数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,4,6,8,10]
<strong>输出:</strong> 7
<strong>解释:</strong> 所有的等差子序列为：
[2,4,6]
[4,6,8]
[6,8,10]
[2,4,6,8]
[4,6,8,10]
[2,4,6,8,10]
[2,6,10]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [7,7,7,7,7]
<strong>输出:</strong> 16
<strong>解释:</strong> 数组中的任意子序列都是等差子序列。
</pre>

#### 提示:
* `1  <= nums.length <= 1000`
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numberOfArithmeticSlices(self, nums: List[int]) -> int:
        count = {}
        ret = 0

        for i in range(1, len(nums)):
            for j in range(i):
                d = nums[i] - nums[j]
                c = count.get((j, d), 0)
                count[(i, d)] = count.get((i, d), 0) + c + 1
                ret += c

        return ret
```
