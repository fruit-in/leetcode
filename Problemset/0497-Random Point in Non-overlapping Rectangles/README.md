# 497. Random Point in Non-overlapping Rectangles
You are given an array of non-overlapping axis-aligned rectangles `rects` where <code>rects[i] = [a<sub>i</sub>, b<sub>i</sub>, x<sub>i</sub>, y<sub>i</sub>]</code> indicates that <code>(a<sub>i</sub>, b<sub>i</sub>)</code> is the bottom-left corner point of the <code>i<sup>th</sup></code> rectangle and <code>(x<sub>i</sub>, y<sub>i</sub>)</code> is the top-right corner point of the <code>i<sup>th</sup></code> rectangle. Design an algorithm to pick a random integer point inside the space covered by one of the given rectangles. A point on the perimeter of a rectangle is included in the space covered by the rectangle.

Any integer point inside the space covered by one of the given rectangles should be equally likely to be returned.

**Note** that an integer point is a point that has integer coordinates.

Implement the `Solution` class:

* `Solution(int[][] rects)` Initializes the object with the given rectangles `rects`.
* `int[] pick()` Returns a random integer point `[u, v]` inside the space covered by one of the given rectangles.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/07/24/lc-pickrandomrec.jpg)
<pre>
<strong>Input:</strong>
["Solution", "pick", "pick", "pick", "pick", "pick"]
[[[[-2, -2, 1, 1], [2, 2, 4, 6]]], [], [], [], [], []]
<strong>Output:</strong>
[null, [1, -2], [1, -1], [-1, -2], [-2, -2], [0, 0]]
<strong>Explanation:</strong>
Solution solution = new Solution([[-2, -2, 1, 1], [2, 2, 4, 6]]);
solution.pick(); // return [1, -2]
solution.pick(); // return [1, -1]
solution.pick(); // return [-1, -2]
solution.pick(); // return [-2, -2]
solution.pick(); // return [0, 0]
</pre>

#### Constraints:
* `1 <= rects.length <= 100`
* `rects[i].length == 4`
* <code>-10<sup>9</sup> <= a<sub>i</sub> < x<sub>i</sub> <= 10<sup>9</sup></code>
* <code>-10<sup>9</sup> <= b<sub>i</sub> < y<sub>i</sub> <= 10<sup>9</sup></code>
* <code>x<sub>i</sub> - a<sub>i</sub> <= 2000</code>
* <code>y<sub>i</sub> - b<sub>i</sub> <= 2000</code>
* All the rectangles do not overlap.
* At most <code>10<sup>4</sup></code> calls will be made to `pick`.

## Solutions (Python)

### 1. Solution
```Python
import random


class Solution:

    def __init__(self, rects: List[List[int]]):
        self.rects = []
        self.start = 0

        for a, b, x, y in rects:
            self.rects.append((self.start, a, b, x, y))
            self.start += (x - a + 1) * (y - b + 1)

    def pick(self) -> List[int]:
        n = random.randint(0, self.start - 1)
        i = bisect.bisect(self.rects, n, key=lambda r: r[0]) - 1
        start, a, b, x, _ = self.rects[i]
        n -= start

        return [a + n % (x - a + 1), b + n // (x - a + 1)]


# Your Solution object will be instantiated and called as such:
# obj = Solution(rects)
# param_1 = obj.pick()
```
