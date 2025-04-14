# 2197. 替换数组中的非互质数
给你一个整数数组 `nums` 。请你对数组执行下述操作：
1. 从 `nums` 中找出 **任意** 两个 **相邻** 的 **非互质** 数。
2. 如果不存在这样的数，**终止** 这一过程。
3. 否则，删除这两个数，并 **替换** 为它们的 **最小公倍数**（Least Common Multiple，LCM）。
4. 只要还能找出两个相邻的非互质数就继续 **重复** 这一过程。

返回修改后得到的 **最终** 数组。可以证明的是，以 **任意** 顺序替换相邻的非互质数都可以得到相同的结果。

生成的测试用例可以保证最终数组中的值 **小于或者等于** <code>10<sup>8</sup></code> 。

两个数字 `x` 和 `y` 满足 **非互质数** 的条件是：`GCD(x, y) > 1` ，其中 `GCD(x, y)` 是 `x` 和 `y` 的 **最大公约数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [6,4,3,2,7,6,2]
<strong>输出:</strong> [12,7,6]
<strong>解释:</strong>
- (6, 4) 是一组非互质数，且 LCM(6, 4) = 12 。得到 nums = [12,3,2,7,6,2] 。
- (12, 3) 是一组非互质数，且 LCM(12, 3) = 12 。得到 nums = [12,2,7,6,2] 。
- (12, 2) 是一组非互质数，且 LCM(12, 2) = 12 。得到 nums = [12,7,6,2] 。
- (6, 2) 是一组非互质数，且 LCM(6, 2) = 6 。得到 nums = [12,7,6] 。
现在，nums 中不存在相邻的非互质数。
因此，修改后得到的最终数组是 [12,7,6] 。
注意，存在其他方法可以获得相同的最终数组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,2,1,1,3,3,3]
<strong>输出:</strong> [2,1,1,3]
<strong>解释:</strong>
- (3, 3) 是一组非互质数，且 LCM(3, 3) = 3 。得到 nums = [2,2,1,1,3,3] 。
- (3, 3) 是一组非互质数，且 LCM(3, 3) = 3 。得到 nums = [2,2,1,1,3] 。
- (2, 2) 是一组非互质数，且 LCM(2, 2) = 2 。得到 nums = [2,1,1,3] 。
现在，nums 中不存在相邻的非互质数。
因此，修改后得到的最终数组是 [2,1,1,3] 。
注意，存在其他方法可以获得相同的最终数组。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>
* 生成的测试用例可以保证最终数组中的值 **小于或者等于** <code>10<sup>8</sup></code> 。

## 题解 (Python)

### 1. 题解
```Python
from math import gcd, lcm


class Solution:
    def replaceNonCoprimes(self, nums: List[int]) -> List[int]:
        stack = []

        for x in nums:
            while stack != [] and gcd(stack[-1], x) > 1:
                x = lcm(stack.pop(), x)
            stack.append(x)

        return stack
```
