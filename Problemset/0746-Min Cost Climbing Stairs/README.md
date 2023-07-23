# 746. Min Cost Climbing Stairs
On a staircase, the ```i```-th step has some non-negative cost ```cost[i]``` assigned (0 indexed).

Once you pay the cost, you can either climb one or two steps. You need to find minimum cost to reach the top of the floor, and you can either start from the step with index 0, or the step with index 1.

#### Example 1:
<pre>
<strong>Input:</strong> cost = [10, 15, 20]
<strong>Output:</strong> 15
<strong>Explanation:</strong> Cheapest is start on cost[1], pay that cost and go to the top.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
<strong>Output:</strong> 6
<strong>Explanation:</strong> Cheapest is start on cost[0], and only step on 1s, skipping cost[3].
</pre>

#### Note:
1. ```cost``` will have a length in the range ```[2, 1000]```.
2. Every ```cost[i]``` will be an integer in the range ```[0, 999]```.

## Solutions (Rust)

### 1. Dynamic Programming
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
