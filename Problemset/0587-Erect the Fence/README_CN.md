# 587. 安装栅栏
给定一个数组 `trees`，其中 <code>trees[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> 表示树在花园中的位置。

你被要求用最短长度的绳子把整个花园围起来，因为绳子很贵。只有把 **所有的树都围起来**，花园才围得很好。

返回*恰好位于围栏周边的树木的坐标*。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/04/24/erect2-plane.jpg)
<pre>
<strong>输入:</strong> trees = [[1,1],[2,2],[2,0],[2,4],[3,3],[4,2]]
<strong>输出:</strong> [[1,1],[2,0],[4,2],[3,3],[2,4]]
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/04/24/erect1-plane.jpg)
<pre>
<strong>输入:</strong> trees = [[1,2],[2,2],[4,2]]
<strong>输出:</strong> [[4,2],[2,2],[1,2]]
</pre>

#### 注意:
* `1 <= trees.length <= 3000`
* `trees[i].length == 2`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= 100</code>
* 所有给定的点都是 **唯一** 的。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trees = trees;
        let mut stack = vec![];
        let mut used = HashSet::new();

        trees.sort_unstable();
        stack.push(trees[0].clone());
        used.insert(trees[0].clone());

        for i in 1..trees.len() {
            let (x0, y0) = (trees[i][0], trees[i][1]);

            while stack.len() > 1 {
                let (x1, y1) = (stack[stack.len() - 1][0], stack[stack.len() - 1][1]);
                let (x2, y2) = (stack[stack.len() - 2][0], stack[stack.len() - 2][1]);

                if (x1 - x2) * (y0 - y1) < (x0 - x1) * (y1 - y2) {
                    used.remove(&vec![x1, y1]);
                    stack.pop();
                } else {
                    break;
                }
            }

            used.insert(vec![x0, y0]);
            stack.push(vec![x0, y0]);
        }
        for i in (0..trees.len() - 1).rev() {
            let (x0, y0) = (trees[i][0], trees[i][1]);

            if i > 0 && used.contains(&vec![x0, y0]) {
                continue;
            }

            while stack.len() > 1 {
                let (x1, y1) = (stack[stack.len() - 1][0], stack[stack.len() - 1][1]);
                let (x2, y2) = (stack[stack.len() - 2][0], stack[stack.len() - 2][1]);

                if (x1 - x2) * (y0 - y1) < (x0 - x1) * (y1 - y2) {
                    used.remove(&vec![x1, y1]);
                    stack.pop();
                } else {
                    break;
                }
            }

            used.insert(vec![x0, y0]);
            stack.push(vec![x0, y0]);
        }

        used.into_iter().collect()
    }
}
```
