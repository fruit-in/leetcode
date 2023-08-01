# 2226. Maximum Candies Allocated to K Children
You are given a **0-indexed** integer array `candies`. Each element in the array denotes a pile of candies of size `candies[i]`. You can divide each pile into any number of **sub piles**, but you **cannot** merge two piles together.

You are also given an integer `k`. You should allocate piles of candies to `k` children such that each child gets the **same** number of candies. Each child can take **at most one** pile of candies and some piles of candies may go unused.

Return *the **maximum number of candies** each child can get*.

#### Example 1:
<pre>
<strong>Input:</strong> candies = [5,8,6], k = 3
<strong>Output:</strong> 5
<strong>Explanation:</strong> We can divide candies[1] into 2 piles of size 5 and 3, and candies[2] into 2 piles of size 5 and 1. We now have five piles of candies of sizes 5, 5, 3, 5, and 1. We can allocate the 3 piles of size 5 to 3 children. It can be proven that each child cannot receive more than 5 candies.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> candies = [2,5], k = 11
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are 11 children but only 7 candies in total, so it is impossible to ensure each child receives at least one candy. Thus, each child gets no candy and the answer is 0.
</pre>

#### Constraints:
* <code>1 <= candies.length <= 10<sup>5</sup></code>
* <code>1 <= candies[i] <= 10<sup>7</sup></code>
* <code>1 <= k <= 10<sup>12</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut l = 1;
        let mut r = 10_000_001;

        while l < r {
            let m = (l + r) / 2;
            let count = candies.iter().map(|&x| (x / m) as i64).sum::<i64>();

            if count < k {
                r = m;
            } else {
                l = m + 1;
            }
        }

        r - 1
    }
}
```
