# 1833. Maximum Ice Cream Bars
It is a sweltering summer day, and a boy wants to buy some ice cream bars.

At the store, there are `n` ice cream bars. You are given an array `costs` of length `n`, where `costs[i]` is the price of the <code>i<sup>th</sup></code> ice cream bar in coins. The boy initially has `coins` coins to spend, and he wants to buy as many ice cream bars as possible.

Return *the **maximum** number of ice cream bars the boy can buy with* `coins` *coins*.

**Note:** The boy can buy the ice cream bars in any order.

#### Example 1:
<pre>
<strong>Input:</strong> costs = [1,3,2,4,1], coins = 7
<strong>Output:</strong> 4
<strong>Explanation:</strong> The boy can buy ice cream bars at indices 0,1,2,4 for a total price of 1 + 3 + 2 + 1 = 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> costs = [10,6,8,7,7,8], coins = 5
<strong>Output:</strong> 0
<strong>Explanation:</strong> The boy cannot afford any of the ice cream bars.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> costs = [1,6,3,1,2,5], coins = 20
<strong>Output:</strong> 6
<strong>Explanation:</strong> The boy can buy all the ice cream bars for a total price of 1 + 6 + 3 + 1 + 2 + 5 = 18.
</pre>

#### Constraints:
* `costs.length == n`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= costs[i] <= 10<sup>5</sup></code>
* <code>1 <= coins <= 10<sup>8</sup></code>

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();

        costs
            .iter()
            .take_while(|&x| {
                coins -= x;
                coins >= 0
            })
            .count() as i32
    }
}
```
