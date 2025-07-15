# 2234. Maximum Total Beauty of the Gardens
Alice is a caretaker of `n` gardens and she wants to plant flowers to maximize the total beauty of all her gardens.

You are given a **0-indexed** integer array `flowers` of size `n`, where `flowers[i]` is the number of flowers already planted in the <code>i<sup>th</sup></code> garden. Flowers that are already planted **cannot** be removed. You are then given another integer `newFlowers`, which is the **maximum** number of flowers that Alice can additionally plant. You are also given the integers `target`, `full`, and `partial`.

A garden is considered **complete** if it has **at least** `target` flowers. The **total beauty** of the gardens is then determined as the **sum** of the following:
* The number of **complete** gardens multiplied by `full`.
* The **minimum** number of flowers in any of the **incomplete** gardens multiplied by `partial`. If there are no incomplete gardens, then this value will be `0`.

Return *the **maximum** total beauty that Alice can obtain after planting at most* `newFlowers` *flowers*.

#### Example 1:
<pre>
<strong>Input:</strong> flowers = [1,3,1,1], newFlowers = 7, target = 6, full = 12, partial = 1
<strong>Output:</strong> 14
<strong>Explanation:</strong> Alice can plant
- 2 flowers in the 0th garden
- 3 flowers in the 1st garden
- 1 flower in the 2nd garden
- 1 flower in the 3rd garden
The gardens will then be [3,6,2,2]. She planted a total of 2 + 3 + 1 + 1 = 7 flowers.
There is 1 garden that is complete.
The minimum number of flowers in the incomplete gardens is 2.
Thus, the total beauty is 1 * 12 + 2 * 1 = 12 + 2 = 14.
No other way of planting flowers can obtain a total beauty higher than 14.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> flowers = [2,4,5,3], newFlowers = 10, target = 5, full = 2, partial = 6
<strong>Output:</strong> 30
<strong>Explanation:</strong> Alice can plant
- 3 flowers in the 0th garden
- 0 flowers in the 1st garden
- 0 flowers in the 2nd garden
- 2 flowers in the 3rd garden
The gardens will then be [5,4,5,5]. She planted a total of 3 + 0 + 0 + 2 = 5 flowers.
There are 3 gardens that are complete.
The minimum number of flowers in the incomplete gardens is 4.
Thus, the total beauty is 3 * 2 + 4 * 6 = 6 + 24 = 30.
No other way of planting flowers can obtain a total beauty higher than 30.
Note that Alice could make all the gardens complete but in this case, she would obtain a lower total beauty.
</pre>

#### Constraints:
* <code>1 <= flowers.length <= 10<sup>5</sup></code>
* <code>1 <= flowers[i], target <= 10<sup>5</sup></code>
* <code>1 <= newFlowers <= 10<sup>10</sup></code>
* <code>1 <= full, partial <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
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
