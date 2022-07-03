# 2001. 可互换矩形的组数
用一个下标从 **0** 开始的二维整数数组 `rectangles` 来表示 `n` 个矩形，其中 <code>rectangles[i] = [width<sub>i</sub>, height<sub>i</sub>]</code> 表示第 `i` 个矩形的宽度和高度。

如果两个矩形 `i` 和 `j`（`i < j`）的宽高比相同，则认为这两个矩形 **可互换** 。更规范的说法是，两个矩形满足 <code>width<sub>i</sub>/height<sub>i</sub> == width<sub>j</sub>/height<sub>j</sub></code>（使用实数除法而非整数除法），则认为这两个矩形 **可互换** 。

计算并返回 `rectangles` 中有多少对 **可互换** 矩形。

#### 示例 1:
<pre>
<strong>输入:</strong> rectangles = [[4,8],[3,6],[10,20],[15,30]]
<strong>输出:</strong> 6
<strong>解释:</strong> 下面按下标（从 0 开始）列出可互换矩形的配对情况：
- 矩形 0 和矩形 1 ：4/8 == 3/6
- 矩形 0 和矩形 2 ：4/8 == 10/20
- 矩形 0 和矩形 3 ：4/8 == 15/30
- 矩形 1 和矩形 2 ：3/6 == 10/20
- 矩形 1 和矩形 3 ：3/6 == 15/30
- 矩形 2 和矩形 3 ：10/20 == 15/30
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> rectangles = [[4,5],[7,8]]
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在成对的可互换矩形。
</pre>

#### 提示:
* `n == rectangles.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `rectangles[i].length == 2`
* <code>1 <= width<sub>i</sub>, height<sub>i</sub> <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
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
