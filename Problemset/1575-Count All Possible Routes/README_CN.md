# 1575. 统计所有可行路径
给你一个 **互不相同** 的整数数组，其中 `locations[i]` 表示第 `i` 个城市的位置。同时给你 `start`，`finish` 和 `fuel` 分别表示出发城市、目的地城市和你初始拥有的汽油总量

每一步中，如果你在城市 `i` ，你可以选择任意一个城市 `j` ，满足  `j != i` 且 `0 <= j < locations.length` ，并移动到城市 `j` 。从城市 `i` 移动到 `j` 消耗的汽油量为 `|locations[i] - locations[j]|`，`|x|` 表示 `x` 的绝对值。

请注意， `fuel` 任何时刻都 **不能** 为负，且你 **可以** 经过任意城市超过一次（包括 `start` 和 `finish` ）。

请你返回从 `start` 到 `finish` 所有可能路径的数目。

由于答案可能很大， 请将它对 `10^9 + 7` 取余后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> locations = [2,3,6,8,4], start = 1, finish = 3, fuel = 5
<strong>输出:</strong> 4
<strong>解释:</strong> 以下为所有可能路径，每一条都用了 5 单位的汽油：
1 -> 3
1 -> 2 -> 3
1 -> 4 -> 3
1 -> 4 -> 2 -> 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> locations = [4,3,1], start = 1, finish = 0, fuel = 6
<strong>输出:</strong> 5
<strong>解释:</strong> 以下为所有可能的路径：
1 -> 0，使用汽油量为 fuel = 1
1 -> 2 -> 0，使用汽油量为 fuel = 5
1 -> 2 -> 1 -> 0，使用汽油量为 fuel = 5
1 -> 0 -> 1 -> 0，使用汽油量为 fuel = 3
1 -> 0 -> 1 -> 0 -> 1 -> 0，使用汽油量为 fuel = 5
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> locations = [5,2,1], start = 0, finish = 2, fuel = 3
<strong>输出:</strong> 0
<strong>解释:</strong> 没有办法只用 3 单位的汽油从 0 到达 2 。因为最短路径需要 4 单位的汽油。
</pre>

#### 提示:
* `2 <= locations.length <= 100`
* <code>1 <= locations[i] <= 10<sup>9</sup></code>
* 所有 `locations` 中的整数 **互不相同** 。
* `0 <= start, finish < locations.length`
* `1 <= fuel <= 200`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let (start, finish) = (start as usize, finish as usize);
        let fuel = fuel as usize;
        let mut dp = vec![vec![0; fuel + 1]; locations.len()];
        dp[start][0] = 1;

        for i in 0..fuel {
            for j in 0..locations.len() {
                for k in 0..locations.len() {
                    let cost = (locations[j] - locations[k]).abs() as usize;

                    if j != k && i + cost <= fuel {
                        dp[k][i + cost] = (dp[k][i + cost] + dp[j][i]) % 1_000_000_007;
                    }
                }
            }
        }

        dp[finish]
            .iter()
            .fold(0, |acc, x| (acc + x) % 1_000_000_007)
    }
}
```
