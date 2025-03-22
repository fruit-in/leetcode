# 2561. Rearranging Fruits
You have two fruit baskets containing `n` fruits each. You are given two **0-indexed** integer arrays `basket1` and `basket2` representing the cost of fruit in each basket. You want to make both baskets **equal**. To do so, you can use the following operation as many times as you want:
* Chose two indices `i` and `j`, and swap the <code>i<sub>th</sub></code> fruit of `basket1` with the <code>j<sub>th</sub></code> fruit of `basket2`.
* The cost of the swap is `min(basket1[i],basket2[j])`.

Two baskets are considered equal if sorting them according to the fruit cost makes them exactly the same baskets.

Return *the minimum cost to make both the baskets equal or* `-1` *if impossible*.

#### Example 1:
<pre>
<strong>Input:</strong> basket1 = [4,2,2,2], basket2 = [1,4,1,2]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Swap index 1 of basket1 with index 0 of basket2, which has cost 1. Now basket1 = [4,1,2,2] and basket2 = [2,4,1,2]. Rearranging both the arrays makes them equal.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> basket1 = [2,3,4,1], basket2 = [3,2,5,1]
<strong>Output:</strong> -1
<strong>Explanation:</strong> It can be shown that it is impossible to make both the baskets equal.
</pre>

#### Constraints:
* `basket1.length == basket2.length`
* <code>1 <= basket1.length <= 10<sup>5</sup></code>
* <code>1 <= basket1[i],basket2[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut count = HashMap::new();
        let mut min_fruit = i64::MAX;
        let mut swap = vec![];
        let mut ret = 0;

        for i in 0..basket1.len() {
            *count.entry(basket1[i]).or_insert(0_i32) += 1;
            *count.entry(basket2[i]).or_insert(0) -= 1;
            min_fruit = min_fruit.min(basket1[i].min(basket2[i]) as i64);
        }

        for (k, v) in count.into_iter() {
            if v % 2 != 0 {
                return -1;
            }

            swap.append(&mut vec![k as i64; v.abs() as usize / 2]);
        }

        swap.sort_unstable();

        swap.iter()
            .take(swap.len() / 2)
            .map(|&cost| cost.min(min_fruit * 2))
            .sum()
    }
}
```
