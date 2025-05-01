# 2517. Maximum Tastiness of Candy Basket
You are given an array of positive integers `price` where `price[i]` denotes the price of the <code>i<sup>th</sup></code> candy and a positive integer `k`.

The store sells baskets of `k` **distinct** candies. The **tastiness** of a candy basket is the smallest absolute difference of the **prices** of any two candies in the basket.

Return *the **maximum** tastiness of a candy basket*.

#### Example 1:
<pre>
<strong>Input:</strong> price = [13,5,1,8,21,2], k = 3
<strong>Output:</strong> 8
<strong>Explanation:</strong> Choose the candies with the prices [13,5,21].
The tastiness of the candy basket is: min(|13 - 5|, |13 - 21|, |5 - 21|) = min(8, 8, 16) = 8.
It can be proven that 8 is the maximum tastiness that can be achieved.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> price = [1,3,1], k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> Choose the candies with the prices [1,3].
The tastiness of the candy basket is: min(|1 - 3|) = min(2) = 2.
It can be proven that 2 is the maximum tastiness that can be achieved.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> price = [7,7,7,7], k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> Choosing any two distinct candies from the candies we have will result in a tastiness of 0.
</pre>

#### Constraints:
* <code>2 <= k <= price.length <= 10<sup>5</sup></code>
* <code>1 <= price[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
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
