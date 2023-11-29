# 1870. Minimum Speed to Arrive on Time
You are given a floating-point number `hour`, representing the amount of time you have to reach the office. To commute to the office, you must take `n` trains in sequential order. You are also given an integer array `dist` of length `n`, where `dist[i]` describes the distance (in kilometers) of the <code>i<sup>th</sup></code> train ride.

Each train can only depart at an integer hour, so you may need to wait in between each train ride.

* For example, if the <code>1<sup>st</sup></code> train ride takes `1.5` hours, you must wait for an additional `0.5` hours before you can depart on the <code>2<sup>nd</sup></code> train ride at the 2 hour mark.

Return *the **minimum positive integer** speed **(in kilometers per hour)** that all the trains must travel at for you to reach the office on time, or* `-1` *if it is impossible to be on time*.

Tests are generated such that the answer will not exceed <code>10<sup>7</sup></code> and `hour` will have at most two digits after the decimal point.

#### Example 1:
<pre>
<strong>Input:</strong> dist = [1,3,2], hour = 6
<strong>Output:</strong> 1
<strong>Explanation:</strong> At speed 1:
- The first train ride takes 1/1 = 1 hour.
- Since we are already at an integer hour, we depart immediately at the 1 hour mark. The second train takes 3/1 = 3 hours.
- Since we are already at an integer hour, we depart immediately at the 4 hour mark. The third train takes 2/1 = 2 hours.
- You will arrive at exactly the 6 hour mark.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> dist = [1,3,2], hour = 2.7
<strong>Output:</strong> 3
<strong>Explanation:</strong> At speed 3:
- The first train ride takes 1/3 = 0.33333 hours.
- Since we are not at an integer hour, we wait until the 1 hour mark to depart. The second train ride takes 3/3 = 1 hour.
- Since we are already at an integer hour, we depart immediately at the 2 hour mark. The third train takes 2/3 = 0.66667 hours.
- You will arrive at the 2.66667 hour mark.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> dist = [1,3,2], hour = 1.9
<strong>Output:</strong> -1
<strong>Explanation:</strong> It is impossible because the earliest the third train can depart is at the 2 hour mark.
</pre>

#### Constraints:
* `n == dist.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= dist[i] <= 10<sup>5</sup></code>
* <code>1 <= hour <= 10<sup>9</sup></code>
* There will be at most two digits after the decimal point in `hour`.

## Solutions (Python)

### 1. Solution
```Python
from math import ceil


class Solution:
    def minSpeedOnTime(self, dist: List[int], hour: float) -> int:
        if len(dist) - 0.999 > hour:
            return -1

        lo = 1
        hi = 10000000

        while lo < hi:
            v = (lo + hi) // 2
            t = sum(ceil(d / v) for d in dist[:-1]) + dist[-1] / v

            if t > hour:
                lo = v + 1
            else:
                hi = v

        return hi
```
