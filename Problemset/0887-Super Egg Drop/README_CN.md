# 887. 鸡蛋掉落
给你 `k` 枚相同的鸡蛋，并可以使用一栋从第 `1` 层到第 `n` 层共有 `n` 层楼的建筑。

已知存在楼层 `f` ，满足 `0 <= f <= n` ，任何从 **高于** `f` 的楼层落下的鸡蛋都会碎，从 `f` 楼层或比它低的楼层落下的鸡蛋都不会破。

每次操作，你可以取一枚没有碎的鸡蛋并把它从任一楼层 `x` 扔下（满足 `1 <= x <= n`）。如果鸡蛋碎了，你就不能再次使用它。如果某枚鸡蛋扔下后没有摔碎，则可以在之后的操作中 **重复使用** 这枚鸡蛋。

请你计算并返回要确定 `f` **确切的值** 的 **最小操作次数** 是多少？

#### 示例 1:
<pre>
<strong>输入:</strong> k = 1, n = 2
<strong>输出:</strong> 2
<strong>解释:</strong>
鸡蛋从 1 楼掉落。如果它碎了，肯定能得出 f = 0 。
否则，鸡蛋从 2 楼掉落。如果它碎了，肯定能得出 f = 1 。
如果它没碎，那么肯定能得出 f = 2 。
因此，在最坏的情况下我们需要移动 2 次以确定 f 是多少。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> k = 2, n = 6
<strong>输出:</strong> 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> k = 3, n = 14
<strong>输出:</strong> 4
</pre>

#### 提示:
* `1 <= k <= 100`
* <code>1 <= n <= 10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    @cache
    def superEggDrop(self, k: int, n: int) -> int:
        if n == 0:
            return 0
        if k == 1:
            return n

        x = bisect_right(range(1, n + 1), 0, key=lambda x: self.superEggDrop(k -
                         1, x - 1) - self.superEggDrop(k, n - x)) + 1
        ret = 1 + self.superEggDrop(k - 1, x - 1)
        if x > 0:
            ret = min(ret, 1 + self.superEggDrop(k, n - x + 1))

        return ret
```
