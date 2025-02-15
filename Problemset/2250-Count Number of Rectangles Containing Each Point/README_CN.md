# 2250. 统计包含每个点的矩形数目
给你一个二维整数数组 `rectangles` ，其中 <code>rectangles[i] = [l<sub>i</sub>, h<sub>i</sub>]</code> 表示第 `i` 个矩形长为 <code>l<sub>i</sub></code> 高为 <code>h<sub>i</sub></code> 。给你一个二维整数数组 `points` ，其中 <code>points[j] = [x<sub>j</sub>, y<sub>j</sub>]</code> 是坐标为 <code>(x<sub>j</sub>, y<sub>j</sub>)</code> 的一个点。

第 `i` 个矩形的 **左下角** 在 `(0, 0)` 处，**右上角** 在 <code>(l<sub>i</sub>, h<sub>i</sub>)</code> 。

请你返回一个整数数组 `count` ，长度为 `points.length`，其中 `count[j]`是 **包含** 第 `j` 个点的矩形数目。

如果 <code>0 <= x<sub>j</sub> <= l<sub>i</sub></code> 且 <code>0 <= y<sub>j</sub> <= h<sub>i</sub></code> ，那么我们说第 `i` 个矩形包含第 `j` 个点。如果一个点刚好在矩形的 **边上** ，这个点也被视为被矩形包含。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/03/02/example1.png)
<pre>
<strong>输入:</strong> rectangles = [[1,2],[2,3],[2,5]], points = [[2,1],[1,4]]
<strong>输出:</strong> [2,1]
<strong>解释:</strong>
第一个矩形不包含任何点。
第二个矩形只包含一个点 (2, 1) 。
第三个矩形包含点 (2, 1) 和 (1, 4) 。
包含点 (2, 1) 的矩形数目为 2 。
包含点 (1, 4) 的矩形数目为 1 。
所以，我们返回 [2, 1] 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/02/example2.png)
<pre>
<strong>输入:</strong> rectangles = [[1,1],[2,2],[3,3]], points = [[1,3],[1,1]]
<strong>输出:</strong> [1,3]
<strong>解释:</strong>
第一个矩形只包含点 (1, 1) 。
第二个矩形只包含点 (1, 1) 。
第三个矩形包含点 (1, 3) 和 (1, 1) 。
包含点 (1, 3) 的矩形数目为 1 。
包含点 (1, 1) 的矩形数目为 3 。
所以，我们返回 [1, 3] 。
</pre>

#### 提示:
* <code>1 <= rectangles.length, points.length <= 5 * 10<sup>4</sup></code>
* `rectangles[i].length == points[j].length == 2`
* <code>1 <= l<sub>i</sub>, x<sub>j</sub> <= 10<sup>9</sup></code>
* <code>1 <= h<sub>i</sub>, y<sub>j</sub> <= 100</code>
* 所有 `rectangles` **互不相同** 。
* 所有 `points` **互不相同** 。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countRectangles(self, rectangles: List[List[int]], points: List[List[int]]) -> List[int]:
        sortedls = [[] for _ in range(max(h for _, h in rectangles) + 1)]
        count = [0] * len(points)

        for l, h in rectangles:
            sortedls[h].append(l)

        for i in range(len(sortedls)):
            sortedls[i].sort()

        for i, [x, y] in enumerate(points):
            for h in range(y, len(sortedls)):
                count[i] += len(sortedls[h]) - bisect_left(sortedls[h], x)

        return count
```
