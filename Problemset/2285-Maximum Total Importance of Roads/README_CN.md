# 2285. 道路的最大总重要性
给你一个整数 `n` ，表示一个国家里的城市数目。城市编号为 `0` 到 `n - 1` 。

给你一个二维整数数组 `roads` ，其中 <code>roads[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示城市 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条 **双向** 道路。

你需要给每个城市安排一个从 `1` 到 `n` 之间的整数值，且每个值只能被使用 **一次** 。道路的 **重要性** 定义为这条道路连接的两座城市数值 **之和** 。

请你返回在最优安排下，**所有道路重要性** 之和 **最大** 为多少。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/04/07/ex1drawio.png)
<pre>
<strong>输入:</strong> n = 5, roads = [[0,1],[1,2],[2,3],[0,2],[1,3],[2,4]]
<strong>输出:</strong> 43
<strong>解释:</strong> 上图展示了国家图和每个城市被安排的值 [2,4,5,3,1] 。
- 道路 (0,1) 重要性为 2 + 4 = 6 。
- 道路 (1,2) 重要性为 4 + 5 = 9 。
- 道路 (2,3) 重要性为 5 + 3 = 8 。
- 道路 (0,2) 重要性为 2 + 5 = 7 。
- 道路 (1,3) 重要性为 4 + 3 = 7 。
- 道路 (2,4) 重要性为 5 + 1 = 6 。
所有道路重要性之和为 6 + 9 + 8 + 7 + 7 + 6 = 43 。
可以证明，重要性之和不可能超过 43 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/04/07/ex2drawio.png)
<pre>
<strong>输入:</strong> n = 5, roads = [[0,3],[2,4],[1,3]]
<strong>输出:</strong> 20
<strong>解释:</strong> 上图展示了国家图和每个城市被安排的值 [4,3,2,5,1] 。
- 道路 (0,3) 重要性为 4 + 5 = 9 。
- 道路 (2,4) 重要性为 2 + 1 = 3 。
- 道路 (1,3) 重要性为 3 + 5 = 8 。
所有道路重要性之和为 9 + 3 + 8 = 20 。
可以证明，重要性之和不可能超过 20 。
</pre>

#### 提示:
* <code>2 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= roads.length <= 5 * 10<sup>4</sup></code>
* `roads[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* 没有重复道路。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut count = vec![0; n as usize];

        for i in 0..roads.len() {
            count[roads[i][0] as usize] += 1;
            count[roads[i][1] as usize] += 1;
        }

        count.sort_unstable();

        count.iter().zip(1..=n as i64).map(|(c, v)| c * v).sum()
    }
}
```
