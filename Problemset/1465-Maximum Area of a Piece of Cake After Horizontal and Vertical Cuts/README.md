# 1465. Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts
You are given a rectangular cake of size `h x w` and two arrays of integers `horizontalCuts` and `verticalCuts` where:
* `horizontalCuts[i]` is the distance from the top of the rectangular cake to the <code>i<sup>th</sup></code> horizontal cut and similarly, and
* `verticalCuts[j]` is the distance from the left of the rectangular cake to the <code>j<sup>th</sup></code> vertical cut.

Return *the maximum area of a piece of cake after you cut at each horizontal and vertical position provided in the arrays* `horizontalCuts` *and* `verticalCuts`. Since the answer can be a large number, return this **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/05/14/leetcode_max_area_2.png)
<pre>
<strong>Input:</strong> h = 5, w = 4, horizontalCuts = [1,2,4], verticalCuts = [1,3]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The figure above represents the given rectangular cake. Red lines are the horizontal and vertical cuts. After you cut the cake, the green piece of cake has the maximum area.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/05/14/leetcode_max_area_3.png)
<pre>
<strong>Input:</strong> h = 5, w = 4, horizontalCuts = [3,1], verticalCuts = [1]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The figure above represents the given rectangular cake. Red lines are the horizontal and vertical cuts. After you cut the cake, the green and yellow pieces of cake have the maximum area.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> h = 5, w = 4, horizontalCuts = [3], verticalCuts = [3]
<strong>Output:</strong> 9
</pre>

#### Constraints:
* <code>2 <= h, w <= 10<sup>9</sup></code>
* <code>1 <= horizontalCuts.length <= min(h - 1, 10<sup>5</sup>)</code>
* <code>1 <= verticalCuts.length <= min(w - 1, 10<sup>5</sup>)</code>
* `1 <= horizontalCuts[i] < h`
* `1 <= verticalCuts[i] < w`
* All the elements in `horizontalCuts` are distinct.
* All the elements in `verticalCuts` are distinct.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maxArea(self, h: int, w: int, horizontalCuts: List[int], verticalCuts: List[int]) -> int:
        horizontalCuts.extend([0, h])
        verticalCuts.extend([0, w])
        horizontalCuts.sort()
        verticalCuts.sort()
        maxh = max(horizontalCuts[i] - horizontalCuts[i - 1]
                   for i in range(1, len(horizontalCuts)))
        maxw = max(verticalCuts[i] - verticalCuts[i - 1]
                   for i in range(1, len(verticalCuts)))

        return maxh * maxw % 1000000007
```
