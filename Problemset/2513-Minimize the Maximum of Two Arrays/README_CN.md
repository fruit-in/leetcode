# 2513. 最小化两个数组中的最大值
给你两个数组 `arr1` 和 `arr2` ，它们一开始都是空的。你需要往它们中添加正整数，使它们满足以下条件：
* `arr1` 包含 `uniqueCnt1` 个 **互不相同** 的正整数，每个整数都 **不能** 被 `divisor1` **整除** 。
* `arr2` 包含 `uniqueCnt2` 个 **互不相同** 的正整数，每个整数都 **不能** 被 `divisor2` **整除** 。
* `arr1` 和 `arr2` 中的元素 **互不相同** 。

给你 `divisor1` ，`divisor2` ，`uniqueCnt1` 和 `uniqueCnt2` ，请你返回两个数组中 **最大元素** 的 **最小值** 。

#### 示例 1:
<pre>
<strong>输入:</strong> divisor1 = 2, divisor2 = 7, uniqueCnt1 = 1, uniqueCnt2 = 3
<strong>输出:</strong> 4
<strong>输出:</strong>
我们可以把前 4 个自然数划分到 arr1 和 arr2 中。
arr1 = [1] 和 arr2 = [2,3,4] 。
可以看出两个数组都满足条件。
最大值是 4 ，所以返回 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> divisor1 = 3, divisor2 = 5, uniqueCnt1 = 2, uniqueCnt2 = 1
<strong>输出:</strong> 3
<strong>输出:</strong>
arr1 = [1,2] 和 arr2 = [3] 满足所有条件。
最大值是 3 ，所以返回 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> divisor1 = 2, divisor2 = 4, uniqueCnt1 = 8, uniqueCnt2 = 2
<strong>输出:</strong> 15
<strong>输出:</strong>
最终数组为 arr1 = [1,3,5,7,9,11,13,15] 和 arr2 = [2,6] 。
上述方案是满足所有条件的最优解。
</pre>

#### 提示:
* <code>2 <= divisor1, divisor2 <= 10<sup>5</sup></code>
* <code>1 <= uniqueCnt1, uniqueCnt2 < 10<sup>9</sup></code>
* <code>2 <= uniqueCnt1 + uniqueCnt2 <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minimizeSet(self, divisor1: int, divisor2: int, uniqueCnt1: int, uniqueCnt2: int) -> int:
        def isPossible(x: int) -> bool:
            divisible1, divisible2, divisible12 = x // divisor1, x // divisor2, x // divisor12
            cnt1 = max(uniqueCnt1 - divisible2 + divisible12, 0)
            cnt2 = max(uniqueCnt2 - divisible1 + divisible12, 0)

            return cnt1 + cnt2 <= x - divisible1 - divisible2 + divisible12

        divisor12 = math.lcm(divisor1, divisor2)

        return bisect.bisect(range((uniqueCnt1 + uniqueCnt2) * 3), False, key=isPossible)
```
