# 1465. 切割后面积最大的蛋糕
矩形蛋糕的高度为 `h` 且宽度为 `w`，给你两个整数数组 `horizontalCuts` 和 `verticalCuts`，其中：
* `horizontalCuts[i]` 是从矩形蛋糕顶部到第  `i` 个水平切口的距离
* `verticalCuts[j]` 是从矩形蛋糕的左侧到第 `j` 个竖直切口的距离

请你按数组 *`horizontalCuts`* 和 *`verticalCuts`* 中提供的水平和竖直位置切割后，请你找出 **面积最大** 的那份蛋糕，并返回其 **面积** 。由于答案可能是一个很大的数字，因此需要将结果 对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/05/14/leetcode_max_area_2.png)
<pre>
<strong>输入:</strong> h = 5, w = 4, horizontalCuts = [1,2,4], verticalCuts = [1,3]
<strong>输出:</strong> 4
<strong>解释:</strong> 上图所示的矩阵蛋糕中，红色线表示水平和竖直方向上的切口。切割蛋糕后，绿色的那份蛋糕面积最大。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/05/14/leetcode_max_area_3.png)
<pre>
<strong>输入:</strong> h = 5, w = 4, horizontalCuts = [3,1], verticalCuts = [1]
<strong>输出:</strong> 6
<strong>解释:</strong> 上图所示的矩阵蛋糕中，红色线表示水平和竖直方向上的切口。切割蛋糕后，绿色和黄色的两份蛋糕面积最大。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> h = 5, w = 4, horizontalCuts = [3], verticalCuts = [3]
<strong>输出:</strong> 9
</pre>

#### 提示:
* <code>2 <= h, w <= 10<sup>9</sup></code>
* <code>1 <= horizontalCuts.length <= min(h - 1, 10<sup>5</sup>)</code>
* <code>1 <= verticalCuts.length <= min(w - 1, 10<sup>5</sup>)</code>
* `1 <= horizontalCuts[i] < h`
* `1 <= verticalCuts[i] < w`
* 题目数据保证 `horizontalCuts` 中的所有元素各不相同
* 题目数据保证 `verticalCuts` 中的所有元素各不相同

## 题解 (Python)

### 1. 题解
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
