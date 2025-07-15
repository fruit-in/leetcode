# 2234. 花园的最大总美丽值
Alice 是 `n` 个花园的园丁，她想通过种花，最大化她所有花园的总美丽值。

给你一个下标从 **0** 开始大小为 `n` 的整数数组 `flowers` ，其中 `flowers[i]` 是第 `i` 个花园里已经种的花的数目。已经种了的花 **不能** 移走。同时给你 `newFlowers` ，表示 Alice 额外可以种花的 **最大数目** 。同时给你的还有整数 `target` ，`full` 和 `partial` 。

如果一个花园有 **至少** `target` 朵花，那么这个花园称为 **完善的** ，花园的 **总美丽值** 为以下分数之 **和** ：
* **完善** 花园数目乘以 `full`.
* 剩余 **不完善** 花园里，花的 **最少数目** 乘以 `partial` 。如果没有不完善花园，那么这一部分的值为 `0` 。

请你返回 Alice 种最多 `newFlowers` 朵花以后，能得到的 **最大** 总美丽值。

#### 示例 1:
<pre>
<strong>输入:</strong> flowers = [1,3,1,1], newFlowers = 7, target = 6, full = 12, partial = 1
<strong>输出:</strong> 14
<strong>解释:</strong> Alice 可以按以下方案种花
- 在第 0 个花园种 2 朵花
- 在第 1 个花园种 3 朵花
- 在第 2 个花园种 1 朵花
- 在第 3 个花园种 1 朵花
花园里花的数目为 [3,6,2,2] 。总共种了 2 + 3 + 1 + 1 = 7 朵花。
只有 1 个花园是完善的。
不完善花园里花的最少数目是 2 。
所以总美丽值为 1 * 12 + 2 * 1 = 12 + 2 = 14 。
没有其他方案可以让花园总美丽值超过 14 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> flowers = [2,4,5,3], newFlowers = 10, target = 5, full = 2, partial = 6
<strong>输出:</strong> 30
<strong>解释:</strong> Alice 可以按以下方案种花
- 在第 0 个花园种 3 朵花
- 在第 1 个花园种 0 朵花
- 在第 2 个花园种 0 朵花
- 在第 3 个花园种 2 朵花
花园里花的数目为 [5,4,5,5] 。总共种了 3 + 0 + 0 + 2 = 5 朵花。
有 3 个花园是完善的。
不完善花园里花的最少数目为 4 。
所以总美丽值为 3 * 2 + 4 * 6 = 6 + 24 = 30 。
没有其他方案可以让花园总美丽值超过 30 。
注意，Alice可以让所有花园都变成完善的，但这样她的总美丽值反而更小。
</pre>

#### 提示:
* <code>1 <= flowers.length <= 10<sup>5</sup></code>
* <code>1 <= flowers[i], target <= 10<sup>5</sup></code>
* <code>1 <= newFlowers <= 10<sup>10</sup></code>
* <code>1 <= full, partial <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def maximumBeauty(self, flowers: List[int], newFlowers: int, target: int, full: int, partial: int) -> int:
        fullflowers = []
        partialflowers = []
        fullbeauty = 0
        maxbeauty = 0

        for flower in sorted(flowers, reverse=True):
            if flower >= target:
                fullbeauty += full
            elif target - flower <= newFlowers:
                fullbeauty += full
                newFlowers -= target - flower
                heappush(fullflowers, flower)
            else:
                heappush(partialflowers, (flower, 1))

        while True:
            while newFlowers > 0 and partialflowers and partialflowers[0][0] < target - 1:
                flower, count = heappop(partialflowers)
                while partialflowers and partialflowers[0][0] == flower:
                    count += heappop(partialflowers)[1]
                if partialflowers and (partialflowers[0][0] - flower) * count <= newFlowers:
                    newFlowers -= (partialflowers[0][0] - flower) * count
                    heappush(partialflowers, (partialflowers[0][0], count))
                elif partialflowers or (target - 1 - flower) * count > newFlowers:
                    flower += newFlowers // count
                    heappush(partialflowers,
                             (flower, count - newFlowers % count))
                    if newFlowers % count != 0:
                        heappush(partialflowers,
                                 (flower + 1, newFlowers % count))
                    newFlowers = 0
                else:
                    heappush(partialflowers, (target - 1, 1))

            beauty = fullbeauty
            if partialflowers:
                beauty += partialflowers[0][0] * partial
            maxbeauty = max(maxbeauty, beauty)

            if not fullflowers:
                break

            newFlowers += target - fullflowers[0]
            heappush(partialflowers, (heappop(fullflowers), 1))
            fullbeauty -= full

        return maxbeauty
```
