# 1819. 序列中不同最大公约数的数目
给你一个由正整数组成的数组 `nums` 。

数字序列的 **最大公约数** 定义为序列中所有整数的共有约数中的最大整数。

* 例如，序列 `[4,6,16]` 的最大公约数是 `2` 。

数组的一个 **子序列** 本质是一个序列，可以通过删除数组中的某些元素（或者不删除）得到。

* 例如，`[2,5,10]` 是 `[1,2,1,2,4,1,5,10]` 的一个子序列。

计算并返回 `nums` 的所有 **非空** 子序列中 **不同** 最大公约数的 **数目** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/03/17/image-1.png)
<pre>
<strong>输入:</strong> nums = [6,10,3]
<strong>输出:</strong> 5
<strong>解释:</strong> 上图显示了所有的非空子序列与各自的最大公约数。
不同的最大公约数为 6 、10 、3 、2 和 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,15,40,5,6]
<strong>输出:</strong> 7
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 2 * 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countDifferentSubsequenceGCDs(self, nums: List[int]) -> int:
        isgcd = [False] * (max(nums) + 1)

        for i in nums:
            isgcd[i] = True

        for i in range(1, len(isgcd) // 2 + 1):
            mingcd = 0

            for j in range(i, len(isgcd), i):
                if isgcd[j]:
                    mingcd = gcd(mingcd, j) if mingcd > 0 else j
                if mingcd == i:
                    isgcd[i] = True
                    break

        return sum(isgcd)
```
