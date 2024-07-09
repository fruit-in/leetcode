# 1806. Minimum Number of Operations to Reinitialize a Permutation
You are given an **even** integer `n`. You initially have a permutation `perm` of size `n` where `perm[i] == i` (**0-indexed**).

In one operation, you will create a new array `arr`, and for each `i`:

* If `i % 2 == 0`, then `arr[i] = perm[i / 2]`.
* If `i % 2 == 1`, then `arr[i] = perm[n / 2 + (i - 1) / 2]`.

You will then assign `arr` to `perm`.

Return *the minimum **non-zero** number of operations you need to perform on* `perm` *to return the permutation to its initial value*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> perm = [0,1] initially.
After the 1st operation, perm = [0,1]
So it takes only 1 operation.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> perm = [0,1,2,3] initially.
After the 1st operation, perm = [0,2,1,3]
After the 2nd operation, perm = [0,1,2,3]
So it takes only 2 operations.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 6
<strong>Output:</strong> 4
</pre>

#### Constraints:
* `2 <= n <= 1000`
* `n` is even.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let n = n as usize;
        let mut perm = (0..n).collect::<Vec<_>>();

        for ret in 1..=n as i32 {
            let mut arr = vec![0; n];

            for i in 0..n {
                if i % 2 == 0 {
                    arr[i] = perm[i / 2];
                } else {
                    arr[i] = perm[n / 2 + (i - 1) / 2];
                }
            }

            perm = arr;

            if perm.iter().enumerate().all(|(i, &x)| i == x) {
                return ret;
            }
        }

        unreachable!()
    }
}
```
