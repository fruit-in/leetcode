# 2517. 礼盒的最大甜蜜度
给你一个正整数数组 `price` ，其中 `price[i]` 表示第 `i` 类糖果的价格，另给你一个正整数 `k` 。

商店组合 `k` 类 **不同** 糖果打包成礼盒出售。礼盒的 **甜蜜度** 是礼盒中任意两种糖果 **价格** 绝对差的最小值。

返回礼盒的 **最大** 甜蜜度。

#### 示例 1:
<pre>
<strong>输入:</strong> price = [13,5,1,8,21,2], k = 3
<strong>输出:</strong> 8
<strong>解释:</strong> 选出价格分别为 [13,5,21] 的三类糖果。
礼盒的甜蜜度为 min(|13 - 5|, |13 - 21|, |5 - 21|) = min(8, 8, 16) = 8 。
可以证明能够取得的最大甜蜜度就是 8 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> price = [1,3,1], k = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 选出价格分别为 [1,3] 的两类糖果。
礼盒的甜蜜度为 min(|1 - 3|) = min(2) = 2 。
可以证明能够取得的最大甜蜜度就是 2 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> price = [7,7,7,7], k = 2
<strong>输出:</strong> 0
<strong>解释:</strong> 从现有的糖果中任选两类糖果，甜蜜度都会是 0 。
</pre>

#### 提示:
* <code>2 <= k <= price.length <= 10<sup>5</sup></code>
* <code>1 <= price[i] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def maximumTastiness(self, price: List[int], k: int) -> int:
        def cannotKCandies(tastiness: int) -> bool:
            i = 0
            count = 0

            while i < len(price) and count < k:
                count += 1
                i = bisect.bisect_left(price, price[i] + tastiness, lo=i + 1)

            return count < k

        price.sort()

        return bisect.bisect(range(price[-1] - price[0] + 1), False, key=cannotKCandies) - 1
```
