# 1824. 最少侧跳次数
给你一个长度为 `n` 的 **3 跑道道路** ，它总共包含 `n + 1` 个 **点** ，编号为 `0` 到 `n` 。一只青蛙从 `0` 号点第二条跑道 **出发** ，它想要跳到点 `n` 处。然而道路上可能有一些障碍。

给你一个长度为 `n + 1` 的数组 `obstacles` ，其中 `obstacles[i]` （**取值范围从 0 到 3**）表示在点 `i` 处的 `obstacles[i]` 跑道上有一个障碍。如果 `obstacles[i] == 0` ，那么点 `i` 处没有障碍。任何一个点的三条跑道中 **最多有一个** 障碍。
* 比方说，如果 `obstacles[2] == 1` ，那么说明在点 2 处跑道 1 有障碍。

这只青蛙从点 `i` 跳到点 `i + 1` 且跑道不变的前提是点 `i + 1` 的同一跑道上没有障碍。为了躲避障碍，这只青蛙也可以在 同一个 点处 **侧跳** 到 **另外一条** 跑道（这两条跑道可以不相邻），但前提是跳过去的跑道该点处没有障碍。
* 比方说，这只青蛙可以从点 3 处的跑道 3 跳到点 3 处的跑道 1 。

这只青蛙从点 0 处跑道 2 出发，并想到达点 `n` 处的 **任一跑道** ，请你返回 **最少侧跳次数** 。

**注意:** 点 `0` 处和点 `n` 处的任一跑道都不会有障碍。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/03/25/ic234-q3-ex1.png)
<pre>
<strong>输入:</strong> obstacles = [0,1,2,3,0]
<strong>输出:</strong> 2
<strong>解释:</strong> 最优方案如上图箭头所示。总共有 2 次侧跳（红色箭头）。
注意，这只青蛙只有当侧跳时才可以跳过障碍（如上图点 2 处所示）。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/03/25/ic234-q3-ex2.png)
<pre>
<strong>输入:</strong> obstacles = [0,1,1,3,3,0]
<strong>输出:</strong> 0
<strong>解释:</strong> 跑道 2 没有任何障碍，所以不需要任何侧跳。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/03/25/ic234-q3-ex3.png)
<pre>
<strong>输入:</strong> obstacles = [0,2,1,0,3,0]
<strong>输出:</strong> 2
<strong>解释:</strong> 最优方案如上图所示。总共有 2 次侧跳。
</pre>

#### 提示:
* `obstacles.length == n + 1`
* <code>1 <= n <= 5 * 10<sup>5</sup></code>
* `0 <= obstacles[i] <= 3`
* `obstacles[0] == obstacles[n] == 0`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let mut dp0 = [1, 0, 1];

        for &obstacle in &obstacles[1..] {
            let mut dp1 = dp0;

            if obstacle > 0 {
                dp1[obstacle as usize - 1] = i32::MAX;
            }

            let min_sj = *dp1.iter().min().unwrap();

            for i in 0..3 {
                if obstacle as usize != i + 1 {
                    dp1[i] = dp1[i].min(min_sj + 1);
                }
            }

            dp0 = dp1;
        }

        *dp0.iter().min().unwrap()
    }
}
```
