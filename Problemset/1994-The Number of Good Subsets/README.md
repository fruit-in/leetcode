# 1994. The Number of Good Subsets
You are given an integer array `nums`. We call a subset of `nums` **good** if its product can be represented as a product of one or more **distinct prime** numbers.

* For example, if `nums = [1, 2, 3, 4]`:
    * `[2, 3]`, `[1, 2, 3]`, and `[1, 3]` are **good** subsets with products `6 = 2*3`, `6 = 2*3`, and `3 = 3` respectively.
    * `[1, 4]` and `[4]` are not **good** subsets with products `4 = 2*2` and `4 = 2*2` respectively.

Return *the number of different **good** subsets in* `nums` ***modulo*** <code>10<sup>9</sup> + 7</code>.

A **subset** of `nums` is any array that can be obtained by deleting some (possibly none or all) elements from `nums`. Two subsets are different if and only if the chosen indices to delete are different.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The good subsets are:
- [1,2]: product is 2, which is the product of distinct prime 2.
- [1,2,3]: product is 6, which is the product of distinct primes 2 and 3.
- [1,3]: product is 3, which is the product of distinct prime 3.
- [2]: product is 2, which is the product of distinct prime 2.
- [2,3]: product is 6, which is the product of distinct primes 2 and 3.
- [3]: product is 3, which is the product of distinct prime 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,2,3,15]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The good subsets are:
- [2]: product is 2, which is the product of distinct prime 2.
- [2,3]: product is 6, which is the product of distinct primes 2 and 3.
- [2,15]: product is 30, which is the product of distinct primes 2, 3, and 5.
- [3]: product is 3, which is the product of distinct prime 3.
- [15]: product is 15, which is the product of distinct primes 3 and 5.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `1 <= nums[i] <= 30`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
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

        ret as i32
    }
}
```
