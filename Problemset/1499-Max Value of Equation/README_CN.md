# 1499. 满足不等式的最大值
给你一个数组 `points` 和一个整数 `k` 。数组中每个元素都表示二维平面上的点的坐标，并按照横坐标 x 的值从小到大排序。也就是说 <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> ，并且在 `1 <= i < j <= points.length` 的前提下， <code>x<sub>i</sub> < x<sub>j</sub></code> 总成立。

请你找出 <code>y<sub>i</sub> + y<sub>j</sub> + |x<sub>i</sub> - x<sub>j</sub>|</code> 的 **最大值**，其中 <code>|x<sub>i</sub> - x<sub>j</sub>| <= k</code> 且 `1 <= i < j <= points.length`。

题目测试数据保证至少存在一对能够满足 <code>|x<sub>i</sub> - x<sub>j</sub>| <= k</code> 的点。

#### 示例 1:
<pre>
<strong>输入:</strong> points = [[1,3],[2,0],[5,10],[6,-10]], k = 1
<strong>输出:</strong> 4
<strong>解释:</strong> 前两个点满足 |xi - xj| <= 1 ，代入方程计算，则得到值 3 + 0 + |1 - 2| = 4 。第三个和第四个点也满足条件，得到值 10 + -10 + |5 - 6| = 1 。
没有其他满足条件的点，所以返回 4 和 1 中最大的那个。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> points = [[0,0],[3,0],[9,2]], k = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 只有前两个点满足 |xi - xj| <= 3 ，代入方程后得到值 0 + 0 + |0 - 3| = 3 。
</pre>

#### 提示:
* <code>2 <= points.length <= 10<sup>5</sup></code>
* `points[i].length == 2`
* <code>-10<sup>8</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>8</sup></code>
* <code>0 <= k <= 2 * 10<sup>8</sup></code>
* 对于所有的`1 <= i < j <= points.length` ，`points[i][0] < points[j][0]` 都成立。也就是说，<code>x<sub>i</sub></code> 是严格递增的。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut deque = VecDeque::new();
        let mut ret = i32::MIN;

        for j in 0..points.len() {
            let (xj, yj) = (points[j][0], points[j][1]);

            while xj - deque.front().unwrap_or(&(xj, 0)).0 > k {
                deque.pop_front();
            }

            if let Some(&(xi, yi)) = deque.front() {
                ret = ret.max(yi + yj + xj - xi);
            }

            while let Some(&(xi, yi)) = deque.back() {
                if yi - xi <= yj - xj {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back((xj, yj));
        }

        ret
    }
}
```
