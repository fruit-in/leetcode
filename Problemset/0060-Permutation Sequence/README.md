# 60. Permutation Sequence
The set `[1, 2, 3, ..., n]` contains a total of `n!` unique permutations.

By listing and labeling all of the permutations in order, we get the following sequence for `n = 3`:

1. `"123"`
2. `"132"`
3. `"213"`
4. `"231"`
5. `"312"`
6. `"321"`

Given `n` and `k`, return the <code>k<sup>th</sup></code> permutation sequence.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3, k = 3
<strong>Output:</strong> "213"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 4, k = 9
<strong>Output:</strong> "2314"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 3, k = 1
<strong>Output:</strong> "123"
</pre>

#### Constraints:
* `1 <= n <= 9`
* `1 <= k <= n!`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut k = k as usize - 1;
        let mut digits = (b'1'..=(n as u8 + b'0')).collect::<Vec<_>>();
        let mut factorials = (0..n).collect::<Vec<_>>();
        let mut ret = vec![b'0'; n];

        factorials[0] = 1;
        for i in 3..n {
            factorials[i] *= factorials[i - 1];
        }

        for i in 0..n {
            ret[i] = digits.remove(k / factorials[n - i - 1]);
            k %= factorials[n - i - 1];
        }

        String::from_utf8(ret).unwrap()
    }
}
```
