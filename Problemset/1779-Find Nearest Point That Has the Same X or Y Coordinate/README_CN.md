# 1779. 找到最近的有相同 X 或 Y 坐标的点
给你两个整数 `x` 和 `y` ，表示你在一个笛卡尔坐标系下的 `(x, y)` 处。同时，在同一个坐标系下给你一个数组 `points` ，其中 <code>points[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示在 <code>(a<sub>i</sub>, b<sub>i</sub>)</code> 处有一个点。当一个点与你所在的位置有相同的 x 坐标或者相同的 y 坐标时，我们称这个点是 **有效的** 。

请返回距离你当前位置 **曼哈顿距离** 最近的 **有效** 点的下标（下标从 **0** 开始）。如果有多个最近的有效点，请返回下标 **最小** 的一个。如果没有有效点，请返回 `-1` 。

两个点 <code>(x<sub>1</sub>, y<sub>1</sub>)</code> 和 <code>(x<sub>2</sub>, y<sub>2</sub>)</code> 之间的 **曼哈顿距离** 为 <code>abs(x<sub>1</sub> - x<sub>2</sub>) + abs(y<sub>1</sub> - y<sub>2</sub>)</code> 。

#### 示例 1:
<pre>
<strong>输入:</strong> x = 3, y = 4, points = [[1,2],[3,1],[2,4],[2,3],[4,4]]
<strong>输出:</strong> 2
<strong>解释:</strong> 所有点中，[3,1]，[2,4] 和 [4,4] 是有效点。有效点中，[2,4] 和 [4,4] 距离你当前位置的曼哈顿距离最小，都为 1 。[2,4] 的下标最小，所以返回 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> x = 3, y = 4, points = [[3,4]]
<strong>输出:</strong> 0
<strong>解释:</strong> 答案可以与你当前所在位置坐标相同。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> x = 3, y = 4, points = [[2,3]]
<strong>输出:</strong> -1
<strong>解释:</strong> 没有有效点。
</pre>

#### 提示:
* <code>1 <= points.length <= 10<sup>4</sup></code>
* `points[i].length == 2`
* <code>1 <= x, y, a<sub>i</sub>, b<sub>i</sub> <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        points
            .iter()
            .enumerate()
            .filter(|(_, p)| p[0] == x || p[1] == y)
            .map(|(i, p)| (i, (p[0] - x).abs() + (p[1] - y).abs()))
            .min_by_key(|&(_, d)| d)
            .map_or(-1, |(i, _)| i as i32)
    }
}
```
