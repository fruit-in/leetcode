# 526. Beautiful Arrangement
Suppose you have `n` integers labeled `1` through `n`. A permutation of those `n` integers `perm` (**1-indexed**) is considered a **beautiful arrangement** if for every `i` (`1 <= i <= n`), **either** of the following is true:

* `perm[i]` is divisible by `i`.
* `i` is divisible by `perm[i]`.

Given an integer `n`, return *the **number** of the **beautiful arrangements** that you can construct*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The first beautiful arrangement is [1,2]:
    - perm[1] = 1 is divisible by i = 1
    - perm[2] = 2 is divisible by i = 2
The second beautiful arrangement is [2,1]:
    - perm[1] = 2 is divisible by i = 1
    - i = 2 is divisible by perm[2] = 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= n <= 15`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        Self::backtracking(n, 1, 0)
    }

    fn backtracking(n: i32, i: i32, bitmask: i32) -> i32 {
        if i > n {
            return 1;
        }

        (1..=n)
            .filter(|p| (bitmask >> p) & 1 == 0 && (p % i == 0 || i % p == 0))
            .map(|p| Self::backtracking(n, i + 1, bitmask | (1 << p)))
            .sum()
    }
}
```
