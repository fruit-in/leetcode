# 1840. 最高建筑高度
在一座城市里，你需要建 `n` 栋新的建筑。这些新的建筑会从 `1` 到 `n` 编号排成一列。

这座城市对这些新建筑有一些规定：

* 每栋建筑的高度必须是一个非负整数。
* 第一栋建筑的高度 **必须** 是 `0` 。
* 任意两栋相邻建筑的高度差 **不能超过**  `1` 。

除此以外，某些建筑还有额外的最高高度限制。这些限制会以二维整数数组 `restrictions` 的形式给出，其中 <code>restrictions[i] = [id<sub>i</sub>, maxHeight<sub>i</sub>]</code> ，表示建筑 <code>id<sub>i</sub></code> 的高度 **不能超过** <code>maxHeight<sub>i</sub></code> 。

题目保证每栋建筑在 `restrictions` 中 **至多出现一次** ，同时建筑 `1` **不会** 出现在 `restrictions` 中。

请你返回 **最高** 建筑能达到的 **最高高度** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex1-1.png)
<pre>
<strong>输入:</strong> n = 5, restrictions = [[2,1],[4,1]]
<strong>输出:</strong> 2
<strong>解释:</strong> 上图中的绿色区域为每栋建筑被允许的最高高度。
我们可以使建筑高度分别为 [0,1,2,1,2] ，最高建筑的高度为 2 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex2.png)
<pre>
<strong>输入:</strong> n = 6, restrictions = []
<strong>输出:</strong> 5
<strong>解释:</strong> 上图中的绿色区域为每栋建筑被允许的最高高度。
我们可以使建筑高度分别为 [0,1,2,3,4,5] ，最高建筑的高度为 5 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex3.png)
<pre>
<strong>输入:</strong> n = 10, restrictions = [[5,3],[2,5],[7,4],[10,3]]
<strong>输出:</strong> 5
<strong>解释:</strong> 上图中的绿色区域为每栋建筑被允许的最高高度。
我们可以使建筑高度分别为 [0,1,2,3,3,4,4,5,4,3] ，最高建筑的高度为 5 。
</pre>

#### 提示:
* <code>2 <= n <= 10<sup>9</sup></code>
* <code>0 <= restrictions.length <= min(n - 1, 10<sup>5</sup>)</code>
* <code>2 <= id<sub>i</sub> <= n</code>
* <code>id<sub>i</sub></code> 是 **唯一的** 。
* <code>0 <= maxHeight<sub>i</sub> <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
        restrictions.push(vec![1, 0]);
        restrictions.sort_unstable();
        if restrictions.last().unwrap()[0] != n {
            restrictions.push(vec![n, n - 1]);
        }

        for i in 1..restrictions.len() {
            restrictions[i][1] = restrictions[i][1]
                .min(restrictions[i - 1][1] + restrictions[i][0] - restrictions[i - 1][0]);
        }
        for i in (0..restrictions.len() - 1).rev() {
            restrictions[i][1] = restrictions[i][1]
                .min(restrictions[i + 1][1] + restrictions[i + 1][0] - restrictions[i][0]);
        }

        (0..restrictions.len() - 1)
            .map(|i| {
                (restrictions[i + 1][1] + restrictions[i][1] - restrictions[i][0]
                    + restrictions[i + 1][0])
                    / 2
            })
            .max()
            .unwrap()
    }
}
```
