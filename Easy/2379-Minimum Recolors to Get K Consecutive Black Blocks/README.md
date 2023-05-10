# 2379. Minimum Recolors to Get K Consecutive Black Blocks
You are given a **0-indexed** string `blocks` of length `n`, where `blocks[i]` is either `'W'` or `'B'`, representing the color of the <code>i<sup>th</sup></code> block. The characters `'W'` and `'B'` denote the colors white and black, respectively.

You are also given an integer `k`, which is the desired number of **consecutive** black blocks.

In one operation, you can **recolor** a white block such that it becomes a black block.

Return *the **minimum** number of operations needed such that there is at least **one** occurrence of* `k` *consecutive black blocks*.

#### Example 1:
<pre>
<strong>Input:</strong> blocks = "WBBWWBBWBW", k = 7
<strong>Output:</strong> 3
<strong>Explanation:</strong>
One way to achieve 7 consecutive black blocks is to recolor the 0th, 3rd, and 4th blocks
so that blocks = "BBBBBBBWBW".
It can be shown that there is no way to achieve 7 consecutive black blocks in less than 3 operations.
Therefore, we return 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> blocks = "WBWBBBW", k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong>
No changes need to be made, since 2 consecutive black blocks already exist.
Therefore, we return 0.
</pre>

#### Constraints:
* `n == blocks.length`
* `1 <= n <= 100`
* `blocks[i]` is either `'W'` or `'B'`.
* `1 <= k <= n`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let mut count_w = blocks.chars().take(k).filter(|&c| c == 'W').count();
        let blocks = blocks.as_bytes();
        let mut ret = count_w;

        for i in k..blocks.len() {
            count_w += (blocks[i] == b'W') as usize - (blocks[i - k] == b'W') as usize;
            ret = ret.min(count_w);
        }

        ret as i32
    }
}
```
