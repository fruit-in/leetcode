# 2411. 按位或最大的最小子数组长度
给你一个长度为 `n` 下标从 **0** 开始的数组 `nums` ，数组中所有数字均为非负整数。对于 `0` 到 `n - 1` 之间的每一个下标 `i` ，你需要找出 `nums` 中一个 **最小** 非空子数组，它的起始位置为 `i` （包含这个位置），同时有 **最大** 的 **按位或运算值** 。

* 换言之，令 <code>B<sup>ij</sup></code> 表示子数组 `nums[i...j]` 的按位或运算的结果，你需要找到一个起始位置为 `i` 的最小子数组，这个子数组的按位或运算的结果等于 <code>max(B<sup>ik</sup>)</code> ，其中 `i <= k <= n - 1` 。

一个数组的按位或运算值是这个数组里所有数字按位或运算的结果。

请你返回一个大小为 `n` 的整数数组 `answer`，其中 `answer[i]`是开始位置为 `i` ，按位或运算结果最大，且 **最短** 子数组的长度。

**子数组** 是数组里一段连续非空元素组成的序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,0,2,1,3]
<strong>输出:</strong> [3,3,2,2,1]
<strong>解释:</strong>
任何位置开始，最大按位或运算的结果都是 3 。
- 下标 0 处，能得到结果 3 的最短子数组是 [1,0,2] 。
- 下标 1 处，能得到结果 3 的最短子数组是 [0,2,1] 。
- 下标 2 处，能得到结果 3 的最短子数组是 [2,1] 。
- 下标 3 处，能得到结果 3 的最短子数组是 [1,3] 。
- 下标 4 处，能得到结果 3 的最短子数组是 [3] 。
所以我们返回 [3,3,2,2,1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2]
<strong>输出:</strong> [2,1]
<strong>解释:</strong>
下标 0 处，能得到最大按位或运算值的最短子数组长度为 2 。
下标 1 处，能得到最大按位或运算值的最短子数组长度为 1 。
所以我们返回 [2,1] 。
</pre>

#### 提示:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def smallestSubarrays(self, nums: List[int]) -> List[int]:
        def subarrayOR(j: int) -> int:
            maxor = 0
            for k in range(m):
                if prefixcount[j + 1][k] - prefixcount[i][k] > 0:
                    maxor |= 1 << k

            return maxor

        m = int(math.log2(max(1, max(nums)))) + 1
        n = len(nums)
        prefixcount = [[0] * m]
        answer = [0] * n

        for x in nums:
            prefixcount.append(prefixcount[-1].copy())

            for i in range(m):
                prefixcount[-1][i] += (x >> i) & 1

        for i in range(n):
            maxor = 0
            for j in range(m):
                if prefixcount[n][j] - prefixcount[i][j] > 0:
                    maxor |= 1 << j

            j = bisect.bisect_left(range(n), maxor, lo=i, key=subarrayOR)
            answer[i] = j - i + 1

        return answer
```
