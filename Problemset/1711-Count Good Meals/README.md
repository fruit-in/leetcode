# 1711. Count Good Meals
A **good meal** is a meal that contains **exactly two different food items** with a sum of deliciousness equal to a power of two.

You can pick **any** two different foods to make a good meal.

Given an array of integers `deliciousness` where `deliciousness[i]` is the deliciousness of the <code>i<sup>th</sup></code> item of food, return *the number of different **good meals** you can make from this list modulo* <code>10<sup>9</sup> + 7</code>.

Note that items with different indices are considered different even if they have the same deliciousness value.

#### Example 1:
<pre>
<strong>Input:</strong> deliciousness = [1,3,5,7,9]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The good meals are (1,3), (1,7), (3,5) and, (7,9).
Their respective sums are 4, 8, 8, and 16, all of which are powers of 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> deliciousness = [1,1,1,3,3,3,7]
<strong>Output:</strong> 15
<strong>Explanation:</strong> The good meals are (1,1) with 3 ways, (1,3) with 9 ways, and (1,7) with 3 ways.
</pre>

#### Constraints:
* <code>1 <= deliciousness.length <= 10<sup>5</sup></code>
* <code>0 <= deliciousness[i] <= 2<sup>20</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0_i64;

        for i in 0..deliciousness.len() {
            count
                .entry(deliciousness[i])
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        for (&food0, c) in count.iter() {
            for i in 0..22 {
                let food1 = (1 << i) - food0;
                if food1 < food0 {
                    ret = (ret + c * count.get(&food1).unwrap_or(&0)) % 1_000_000_007;
                } else if food1 == food0 {
                    ret = (ret + c * (c - 1) / 2) % 1_000_000_007;
                } else {
                    break;
                }
            }
        }

        ret as i32
    }
}
```
