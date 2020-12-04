# 962. Maximum Width Ramp
Given an array `A` of integers, a *ramp* is a tuple `(i, j)` for which `i < j` and `A[i] <= A[j]`.  The width of such a ramp is `j - i`.

Find the maximum width of a ramp in `A`.  If one doesn't exist, return 0.

#### Example 1:
<pre>
<strong>Input:</strong> [6,0,8,2,1,5]
<strong>Output:</strong> 4
<strong>Explanation:</strong>
The maximum width ramp is achieved at (i, j) = (1, 5): A[1] = 0 and A[5] = 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [9,8,1,0,1,9,4,0,4,1]
<strong>Output:</strong> 7
<strong>Explanation:</strong>
The maximum width ramp is achieved at (i, j) = (2, 9): A[2] = 1 and A[9] = 1.
</pre>

#### Note:
1. `2 <= A.length <= 50000`
2. `0 <= A[i] <= 50000`

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn max_width_ramp(a: Vec<i32>) -> i32 {
        let mut v = a
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<Vec<_>>();
        let mut min_i = a.len();
        let mut ret = 0;

        v.sort_unstable();

        for (_, i) in v {
            ret = ret.max(i.saturating_sub(min_i));
            min_i = min_i.min(i);
        }

        ret as i32
    }
}
```
