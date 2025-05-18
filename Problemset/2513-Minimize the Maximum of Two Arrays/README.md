# 2513. Minimize the Maximum of Two Arrays
We have two arrays `arr1` and `arr2` which are initially empty. You need to add positive integers to them such that they satisfy all the following conditions:
* `arr1` contains `uniqueCnt1` **distinct** positive integers, each of which is **not divisible** by `divisor1`.
* `arr2` contains `uniqueCnt2` **distinct** positive integers, each of which is **not divisible** by `divisor2`.
* **No** integer is present in both `arr1` and `arr2`.

Given `divisor1`, `divisor2`, `uniqueCnt1`, and `uniqueCnt2`, return *the **minimum possible maximum** integer that can be present in either array*.

#### Example 1:
<pre>
<strong>Input:</strong> divisor1 = 2, divisor2 = 7, uniqueCnt1 = 1, uniqueCnt2 = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong>
We can distribute the first 4 natural numbers into arr1 and arr2.
arr1 = [1] and arr2 = [2,3,4].
We can see that both arrays satisfy all the conditions.
Since the maximum value is 4, we return it.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> divisor1 = 3, divisor2 = 5, uniqueCnt1 = 2, uniqueCnt2 = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong>
Here arr1 = [1,2], and arr2 = [3] satisfy all conditions.
Since the maximum value is 3, we return it.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> divisor1 = 2, divisor2 = 4, uniqueCnt1 = 8, uniqueCnt2 = 2
<strong>Output:</strong> 15
<strong>Explanation:</strong>
Here, the final possible arrays can be arr1 = [1,3,5,7,9,11,13,15], and arr2 = [2,6].
It can be shown that it is not possible to obtain a lower maximum satisfying all conditions.
</pre>

#### Constraints:
* <code>2 <= divisor1, divisor2 <= 10<sup>5</sup></code>
* <code>1 <= uniqueCnt1, uniqueCnt2 < 10<sup>9</sup></code>
* <code>2 <= uniqueCnt1 + uniqueCnt2 <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
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
