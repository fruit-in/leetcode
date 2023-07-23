# 2001. Number of Pairs of Interchangeable Rectangles
You are given `n` rectangles represented by a **0-indexed** 2D integer array `rectangles`, where <code>rectangles[i] = [width<sub>i</sub>, height<sub>i</sub>]</code> denotes the width and height of the <code>i<sup>th</sup></code> rectangle.

Two rectangles `i` and `j` (`i < j`) are considered **interchangeable** if they have the **same** width-to-height ratio. More formally, two rectangles are **interchangeable** if <code>width<sub>i</sub>/height<sub>i</sub> == width<sub>j</sub>/height<sub>j</sub></code> (using decimal division, not integer division).

Return *the **number** of pairs of **interchangeable** rectangles in* `rectangles`.

#### Example 1:
<pre>
<strong>Input:</strong> rectangles = [[4,8],[3,6],[10,20],[15,30]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The following are the interchangeable pairs of rectangles by index (0-indexed):
- Rectangle 0 with rectangle 1: 4/8 == 3/6.
- Rectangle 0 with rectangle 2: 4/8 == 10/20.
- Rectangle 0 with rectangle 3: 4/8 == 15/30.
- Rectangle 1 with rectangle 2: 3/6 == 10/20.
- Rectangle 1 with rectangle 3: 3/6 == 15/30.
- Rectangle 2 with rectangle 3: 10/20 == 15/30.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> rectangles = [[4,5],[7,8]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no interchangeable pairs of rectangles.
</pre>

#### Constraints:
* `n == rectangles.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `rectangles[i].length == 2`
* <code>1 <= width<sub>i</sub>, height<sub>i</sub> <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def interchangeableRectangles(self, rectangles: List[List[int]]) -> int:
        count = {}

        for width, height in rectangles:
            x = gcd(width, height)
            width, height = width // x, height // x
            if (width, height) not in count:
                count[(width, height)] = 0
            count[(width, height)] += 1

        return sum(x * (x - 1) // 2 for x in count.values())
```
