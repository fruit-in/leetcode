# 793. Preimage Size of Factorial Zeroes Function
Let `f(x)` be the number of zeroes at the end of `x!`. Recall that `x! = 1 * 2 * 3 * ... * x` and by convention, `0! = 1`.

* For example, `f(3) = 0` because `3! = 6` has no zeroes at the end, while `f(11) = 2` because `11! = 39916800` has two zeroes at the end.

Given an integer `k`, return the number of non-negative integers `x` have the property that `f(x) = k`.

#### Example 1:
<pre>
<strong>Input:</strong> k = 0
<strong>Output:</strong> 5
<strong>Explanation:</strong> 0!, 1!, 2!, 3!, and 4! end with k = 0 zeroes.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> k = 5
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no x such that x! ends in k = 5 zeroes.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> k = 3
<strong>Output:</strong> 5
</pre>

#### Constraints:
* <code>0 <= k <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        fn f(x: i64) -> i32 {
            let mut pow5 = 5;
            let mut ret = 0;

            while pow5 <= x {
                ret += x / pow5;
                pow5 *= 5;
            }

            ret as i32
        }

        let mut lo = 0;
        let mut hi = 800000004;

        while lo < hi {
            let mid = (lo + hi) / 2;
            let zeros = f(mid * 5);

            if zeros <= k {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        5 * (f((hi - 1) * 5) == k) as i32
    }
}
```
