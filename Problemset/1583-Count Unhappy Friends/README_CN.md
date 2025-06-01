# 1583. 统计不开心的朋友
给你一份 `n` 位朋友的亲近程度列表，其中 `n` 总是 **偶数** 。

对每位朋友 `i`，`preferences[i]` 包含一份 **按亲近程度从高到低排列** 的朋友列表。换句话说，排在列表前面的朋友与 `i` 的亲近程度比排在列表后面的朋友更高。每个列表中的朋友均以 `0` 到 `n-1` 之间的整数表示。

所有的朋友被分成几对，配对情况以列表 `pairs` 给出，其中 <code>pairs[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> 表示 <code>x<sub>i</sub></code> 与 <code>y<sub>i</sub></code> 配对，且 <code>y<sub>i</sub></code> 与 <code>x<sub>i</sub></code> 配对。

但是，这样的配对情况可能会使其中部分朋友感到不开心。在 `x` 与 `y` 配对且 `u` 与 `v` 配对的情况下，如果同时满足下述两个条件，`x` 就会不开心：
* `x` 与 `u` 的亲近程度胜过 `x` 与 `y`，且
* `u` 与 `x` 的亲近程度胜过 `u` 与 `v`

返回 **不开心的朋友的数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 4, preferences = [[1, 2, 3], [3, 2, 0], [3, 1, 0], [1, 2, 0]], pairs = [[0, 1], [2, 3]]
<strong>输出:</strong> 2
<strong>解释:</strong>
朋友 1 不开心，因为：
- 1 与 0 配对，但 1 与 3 的亲近程度比 1 与 0 高，且
- 3 与 1 的亲近程度比 3 与 2 高。
朋友 3 不开心，因为：
- 3 与 2 配对，但 3 与 1 的亲近程度比 3 与 2 高，且
- 1 与 3 的亲近程度比 1 与 0 高。
朋友 0 和 2 都是开心的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2, preferences = [[1], [0]], pairs = [[1, 0]]
<strong>输出:</strong> 0
<strong>解释:</strong> 朋友 0 和 1 都开心。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 4, preferences = [[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]], pairs = [[1, 3], [0, 2]]
<strong>输出:</strong> 4
</pre>

#### 提示:
* `2 <= n <= 500`
* `n` 是偶数
* `preferences.length == n`
* `preferences[i].length == n - 1`
* `0 <= preferences[i][j] <= n - 1`
* `preferences[i]` 不包含 `i`
* `preferences[i]` 中的所有值都是独一无二的
* `pairs.length == n/2`
* `pairs[i].length == 2`
* <code>x<sub>i</sub> != y<sub>i</sub></code>
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= n - 1</code>
* 每位朋友都 **恰好** 被包含在一对中

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut rank = vec![vec![n; n]; n];
        let mut is_unhappy = vec![false; n];

        for i in 0..n {
            for j in 0..n - 1 {
                rank[i][preferences[i][j] as usize] = j;
            }
        }

        for i in 0..n / 2 {
            let (x, y) = (pairs[i][0] as usize, pairs[i][1] as usize);

            for j in 0..n / 2 {
                if is_unhappy[x] && is_unhappy[y] {
                    break;
                }

                let (u, v) = (pairs[j][0] as usize, pairs[j][1] as usize);

                is_unhappy[x] |= rank[x][u] < rank[x][y] && rank[u][x] < rank[u][v];
                is_unhappy[x] |= rank[x][v] < rank[x][y] && rank[v][x] < rank[v][u];
                is_unhappy[y] |= rank[y][u] < rank[y][x] && rank[u][y] < rank[u][v];
                is_unhappy[y] |= rank[y][v] < rank[y][x] && rank[v][y] < rank[v][u];
            }
        }

        is_unhappy.iter().filter(|&&unhappy| unhappy).count() as i32
    }
}
```
