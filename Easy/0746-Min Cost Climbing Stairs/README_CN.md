# 746. 使用最小花费爬楼梯
数组的每个索引做为一个阶梯，第 ```i```个阶梯对应着一个非负数的体力花费值 ```cost[i]```(索引从0开始)。

每当你爬上一个阶梯你都要花费对应的体力花费值，然后你可以选择继续爬一个阶梯或者爬两个阶梯。

您需要找到达到楼层顶部的最低花费。在开始时，你可以选择从索引为 0 或 1 的元素作为初始阶梯。

#### 示例 1:
<pre>
<strong>输入:</strong> cost = [10, 15, 20]
<strong>输出:</strong> 15
<strong>解释:</strong> 最低花费是从cost[1]开始，然后走两步即可到阶梯顶，一共花费15。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
<strong>输出:</strong> 6
<strong>解释:</strong> 最低花费方式是从cost[0]开始，逐个经过那些1，跳过cost[3]，一共花费6。
</pre>

#### 注意:
1. ```cost``` 的长度将会在 ```[2, 1000]```。
2. 每一个 ```cost[i]``` 将会是一个Integer类型，范围为 ```[0, 999]```。

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = (cost[0], cost[1]);

        for i in 2..cost.len() {
            dp = (dp.1, dp.0.min(dp.1) + cost[i]);
        }

        dp.0.min(dp.1)
    }
}
```
