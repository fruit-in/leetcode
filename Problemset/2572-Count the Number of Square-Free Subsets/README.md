# 2572. Count the Number of Square-Free Subsets
You are given a positive integer **0-indexed** array `nums`.

A subset of the array `nums` is **square-free** if the product of its elements is a **square-free integer**.

A **square-free integer** is an integer that is divisible by no square number other than `1`.

Return *the number of square-free non-empty subsets of the array* **nums**. Since the answer may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

A **non-empty subset** of `nums` is an array that can be obtained by deleting some (possibly none but not all) elements from `nums`. Two subsets are different if and only if the chosen indices to delete are different.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,4,4,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are 3 square-free subsets in this example:
- The subset consisting of the 0th element [3]. The product of its elements is 3, which is a square-free integer.
- The subset consisting of the 3rd element [5]. The product of its elements is 5, which is a square-free integer.
- The subset consisting of 0th and 3rd elements [3,5]. The product of its elements is 15, which is a square-free integer.
It can be proven that there are no more than 3 square-free subsets in the given array.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is 1 square-free subset in this example:
- The subset consisting of the 0th element [1]. The product of its elements is 1, which is a square-free integer.
It can be proven that there is no more than 1 square-free subset in the given array.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `1 <= nums[i] <= 30`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn square_free_subsets(nums: Vec<i32>) -> i32 {
        let m = *nums.iter().max().unwrap() as usize;
        let mut count = vec![0; m + 1];
        let mut primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        primes.retain(|&x| x <= m);
        let n = primes.len();
        let mut dp = vec![0; 1 << n];
        dp[0] = 1_i64;
        let mut ret = 0;

        for &x in &nums {
            if x % 4 != 0 && x % 9 != 0 && x % 25 != 0 {
                count[x as usize] += 1;
            }
        }

        for x in 2..=m {
            if count[x] == 0 {
                continue;
            }

            let mut tmp = dp.clone();
            let mut mask = 0;

            for i in 0..n {
                if x % primes[i] == 0 {
                    mask |= 1 << i;
                }
            }
            for i in 0..1 << n {
                if mask & i == 0 {
                    dp[mask | i] = (dp[mask | i] + tmp[i] * count[x]) % 1_000_000_007;
                }
            }
        }

        for i in 1..1 << n {
            ret = (ret + dp[i]) % 1_000_000_007;
        }
        for _ in 0..count[1] {
            ret = (ret * 2) % 1_000_000_007;
        }
        ret = (ret + (0..count[1]).fold(1, |acc, _| acc * 2 % 1_000_000_007) - 1) % 1_000_000_007;

        ret as i32
    }
}
```
