# 216. Combination Sum III
Find all valid combinations of `k` numbers that sum up to `n` such that the following conditions are true:

* Only numbers `1` through `9` are used.
* Each number is used **at most once**.

Return *a list of all possible valid combinations*. The list must not contain the same combination twice, and the combinations may be returned in any order.

#### Example 1:
<pre>
<strong>Input:</strong> k = 3, n = 7
<strong>Output:</strong> [[1,2,4]]
<strong>Explanation:</strong>
1 + 2 + 4 = 7
There are no other valid combinations.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> k = 3, n = 9
<strong>Output:</strong> [[1,2,6],[1,3,5],[2,3,4]]
<strong>Explanation:</strong>
1 + 2 + 6 = 9
1 + 3 + 5 = 9
2 + 3 + 4 = 9
There are no other valid combinations.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> k = 4, n = 1
<strong>Output:</strong> []
<strong>Explanation:</strong> There are no valid combinations.
Using 4 different numbers in the range [1,9], the smallest sum we can get is 1+2+3+4 = 10 and since 10 > 1, there are no valid combination.
</pre>

#### Constraints:
* `2 <= k <= 9`
* `1 <= n <= 60`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut x: i32 = (1 << k) - 1;
        let mut ret = vec![];

        while x < (1 << 9) {
            let comb = (1..=9)
                .filter(|&digit| (1 << (digit - 1)) & x != 0)
                .collect::<Vec<i32>>();

            if comb.iter().sum::<i32>() == n {
                ret.push(comb);
            }

            x += (x & -x) + (1 << ((x >> x.trailing_zeros()).trailing_ones() - 1)) - 1;
        }

        ret
    }
}
```
