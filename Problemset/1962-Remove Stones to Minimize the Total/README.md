# 1962. Remove Stones to Minimize the Total
You are given a **0-indexed** integer array `piles`, where `piles[i]` represents the number of stones in the <code>i<sup>th</sup></code> pile, and an integer `k`. You should apply the following operation **exactly** `k` times:
* Choose any `piles[i]` and **remove** `floor(piles[i] / 2)` stones from it.

**Notice** that you can apply the operation on the **same** pile more than once.

Return *the **minimum** possible total number of stones remaining after applying the* `k` *operations*.

`floor(x)` is the **greatest** integer that is **smaller** than or **equal** to `x` (i.e., rounds `x` down).

#### Example 1:
<pre>
<strong>Input:</strong> piles = [5,4,9], k = 2
<strong>Output:</strong> 12
<strong>Explanation:</strong> Steps of a possible scenario are:
- Apply the operation on pile 2. The resulting piles are [5,4,5].
- Apply the operation on pile 0. The resulting piles are [3,4,5].
The total number of stones in [3,4,5] is 12.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> piles = [4,3,6,7], k = 3
<strong>Output:</strong> 12
<strong>Explanation:</strong> Steps of a possible scenario are:
- Apply the operation on pile 2. The resulting piles are [4,3,3,7].
- Apply the operation on pile 3. The resulting piles are [4,3,3,4].
- Apply the operation on pile 0. The resulting piles are [2,3,3,4].
The total number of stones in [2,3,3,4] is 12.
</pre>

#### Constraints:
* <code>1 <= piles.length <= 10<sup>5</sup></code>
* <code>1 <= piles[i] <= 10<sup>4</sup></code>
* <code>1 <= k <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut piles = BinaryHeap::from(piles);

        for _ in 0..k {
            let x = piles.pop().unwrap();

            piles.push((x + 1) / 2);
        }

        piles.into_iter().sum()
    }
}
```
