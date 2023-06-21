# 2483. 商店的最少代价
给你一个顾客访问商店的日志，用一个下标从 **0** 开始且只包含字符 `'N'` 和 `'Y'` 的字符串 `customers` 表示：

* 如果第 `i` 个字符是 `'Y'` ，它表示第 `i` 小时有顾客到达。
* 如果第 `i` 个字符是 `'N'` ，它表示第 `i` 小时没有顾客到达。

如果商店在第 `j` 小时关门（`0 <= j <= n`），代价按如下方式计算：

* 在开门期间，如果某一个小时没有顾客到达，代价增加 `1` 。
* 在关门期间，如果某一个小时有顾客到达，代价增加 `1` 。

请你返回在确保代价 **最小** 的前提下，商店的 **最早** 关门时间。

注意，商店在第 `j` 小时关门表示在第 `j` 小时以及之后商店处于关门状态。

#### 示例 1:
<pre>
<strong>输入:</strong> customers = "YYNY"
<strong>输出:</strong> 2
<strong>解释:</strong>
- 第 0 小时关门，总共 1+1+0+1 = 3 代价。
- 第 1 小时关门，总共 0+1+0+1 = 2 代价。
- 第 2 小时关门，总共 0+0+0+1 = 1 代价。
- 第 3 小时关门，总共 0+0+1+1 = 2 代价。
- 第 4 小时关门，总共 0+0+1+0 = 1 代价。
在第 2 或第 4 小时关门代价都最小。由于第 2 小时更早，所以最优关门时间是 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> customers = "NNNNN"
<strong>输出:</strong> 0
<strong>解释:</strong> 最优关门时间是 0 ，因为自始至终没有顾客到达。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> customers = "YYYY"
<strong>输出:</strong> 4
<strong>解释:</strong> 最优关门时间是 4 ，因为每一小时均有顾客到达。
</pre>

#### 提示:
* <code>1 <= customers.length <= 10<sup>5</sup></code>
* `customers` 只包含字符 `'Y'` 和 `'N'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut count_n = 0;
        let mut count_y = customers.chars().filter(|&c| c == 'Y').count();
        let mut min_penalty = count_n + count_y;
        let mut ret = 0;

        for (i, c) in customers.chars().enumerate() {
            if c == 'N' {
                count_n += 1;
            } else if c == 'Y' {
                count_y -= 1;
            }

            if min_penalty > count_n + count_y {
                min_penalty = count_n + count_y;
                ret = i + 1;
            }
        }

        ret as i32
    }
}
```
