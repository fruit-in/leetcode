# 779. K-th Symbol in Grammar
On the first row, we write a `0`. Now in every subsequent row, we look at the previous row and replace each occurrence of `0` with `01`, and each occurrence of `1` with `10`.

Given row `N` and index `K`, return the `K`-th indexed symbol in row `N`. (The values of `K` are 1-indexed.) (1 indexed).

<pre>
<b>Example:</b>
<b>Input:</b> N = 1, K = 1
<b>Output:</b> 0

<b>Input:</b> N = 2, K = 1
<b>Output:</b> 0

<b>Input:</b> N = 2, K = 2
<b>Output:</b> 1

<b>Input:</b> N = 4, K = 5
<b>Output:</b> 1

<b>Explanation:</b>
row 1: 0
row 2: 01
row 3: 0110
row 4: 01101001
</pre>

#### Note:
1. `N` will be an integer in the range `[1, 30]`.
2. `K` will be an integer in the range `[1, 2^(N-1)]`.

## Solutions (Rust)

### 1. Recursion
```Rust
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n <= 2 {
            k - 1
        } else if k > 2_i32.pow(n as u32 - 2) {
            1 - Self::kth_grammar(n - 1, k - 2_i32.pow(n as u32 - 2))
        } else {
            Self::kth_grammar(n - 1, k)
        }
    }
}
```
