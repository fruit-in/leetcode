# 1584. 连接所有点的最小费用
给你一个`points` 数组，表示 2D 平面上的一些点，其中 <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> 。

连接点 <code>[x<sub>i</sub>, y<sub>i</sub>]</code> 和点 <code>[x<sub>j</sub>, y<sub>j</sub>]</code> 的费用为它们之间的 **曼哈顿距离** ：<code>|x<sub>i</sub> - x<sub>j</sub>| + |y<sub>i</sub> - y<sub>j</sub>|</code> ，其中 `|val|` 表示 `val` 的绝对值。

请你返回将所有点连接的最小总费用。只有任意两点之间 **有且仅有** 一条简单路径时，才认为所有点都已连接。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/08/26/d.png)
<pre>
<strong>输入:</strong> points = [[0,0],[2,2],[3,10],[5,2],[7,0]]
<strong>输出:</strong> 20
<strong>解释:</strong>
<img src="https://assets.leetcode.com/uploads/2020/08/26/c.png">
我们可以按照上图所示连接所有点得到最小总费用，总费用为 20 。
注意到任意两个点之间只有唯一一条路径互相到达。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> points = [[3,12],[-2,5],[-4,1]]
<strong>输出:</strong> 18
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> points = [[0,0],[1,1],[1,0],[-1,1]]
<strong>输出:</strong> 4
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> points = [[-1000000,-1000000],[1000000,1000000]]
<strong>输出:</strong> 4000000
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> points = [[0,0]]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= points.length <= 1000`
* <code>-10<sup>6</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>6</sup></code>
* 所有点 <code>(x<sub>i</sub>, y<sub>i</sub>)</code> 两两不同。

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut used = vec![false; points.len()];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let mut ret = 0;

        for _ in 0..points.len() {
            while let Some((Reverse(dist), i)) = heap.pop() {
                if !used[i] {
                    let (xi, yi) = (points[i][0], points[i][1]);
                    used[i] = true;
                    ret += dist;
                    for j in 0..points.len() {
                        if !used[j] {
                            let (xj, yj) = (points[j][0], points[j][1]);
                            let dist = (xi - xj).abs() + (yi - yj).abs();
                            heap.push((Reverse(dist), j));
                        }
                    }
                    break;
                }
            }
        }

        ret
    }
}
```
