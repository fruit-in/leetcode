# 1449. 数位成本和为目标值的最大数字
给你一个整数数组 `cost` 和一个整数 `target` 。请你返回满足如下规则可以得到的 **最大** 整数：

* 给当前结果添加一个数位（`i + 1`）的成本为 `cost[i]` （`cost` 数组下标从 0 开始）。
* 总成本必须恰好等于 `target` 。
* 添加的数位中没有数字 `0` 。

由于答案可能会很大，请你以字符串形式返回。

如果按照上述要求无法得到任何整数，请你返回 "0" 。

#### 示例 1:
<pre>
<strong>输入:</strong> cost = [4,3,2,5,6,7,2,5,5], target = 9
<strong>输出:</strong> "7772"
<strong>解释:</strong> 添加数位 '7' 的成本为 2 ，添加数位 '2' 的成本为 3 。所以 "7772" 的代价为 2*3+ 3*1 = 9 。 "977" 也是满足要求的数字，但 "7772" 是较大的数字。
 数字     成本
  1  ->   4
  2  ->   3
  3  ->   2
  4  ->   5
  5  ->   6
  6  ->   7
  7  ->   2
  8  ->   5
  9  ->   5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> cost = [7,6,5,5,5,6,8,7,8], target = 12
<strong>输出:</strong> "85"
<strong>解释:</strong> 添加数位 '8' 的成本是 7 ，添加数位 '5' 的成本是 5 。"85" 的成本为 7 + 5 = 12 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> cost = [2,4,6,2,4,6,4,4,4], target = 5
<strong>输出:</strong> "0"
<strong>解释:</strong> 总成本是 target 的条件下，无法生成任何整数。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> cost = [6,10,15,40,40,40,40,40,40], target = 47
<strong>输出:</strong> "32211"
</pre>

#### 提示:
* `cost.length == 9`
* `1 <= cost[i] <= 5000`
* `1 <= target <= 5000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let target = target as usize;
        let cost = cost.iter().map(|&x| x as usize).collect::<Vec<_>>();
        let mut dp = vec![[-1; 10]; target + 1];
        dp[0] = [0; 10];

        for i in 0..=target {
            if dp[i][0] == -1 {
                continue;
            }

            for j in 0..9 {
                let mut count = dp[i];
                count[9 - j] += 1;
                count[0] += 1;

                if i + cost[j] <= target && dp[i + cost[j]] < count {
                    dp[i + cost[j]] = count;
                }
            }
        }

        if dp[target][0] == -1 {
            return "0".to_string();
        }

        (0..9)
            .rev()
            .map(|i| vec![std::char::from_u32(49 + i as u32).unwrap(); dp[target][9 - i] as usize])
            .flatten()
            .collect()
    }
}
```
