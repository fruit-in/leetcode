# 1734. Decode XORed Permutation
There is an integer array `perm` that is a permutation of the first `n` positive integers, where `n` is always **odd**.

It was encoded into another integer array `encoded` of length `n - 1`, such that `encoded[i] = perm[i] XOR perm[i + 1]`. For example, if `perm = [1,3,2]`, then `encoded = [2,1]`.

Given the `encoded` array, return *the original array* `perm`. It is guaranteed that the answer exists and is unique.

#### Example 1:
<pre>
<strong>Input:</strong> encoded = [3,1]
<strong>Output:</strong> [1,2,3]
<strong>Explanation:</strong> If perm = [1,2,3], then encoded = [1 XOR 2,2 XOR 3] = [3,1]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> encoded = [6,5,4,6]
<strong>Output:</strong> [2,4,1,5,3]
</pre>

#### Constraints:
* <code>3 <= n < 10<sup>5</sup></code>
* `n` is odd.
* `encoded.length == n - 1`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let m = n / 2;
        let mut perm = vec![0; n];
        perm[m] = m as i32 + 1;

        for i in 0..m {
            perm[m] ^= ((i + 1) ^ (n - i)) as i32;
            if i % 2 == 0 {
                perm[m] ^= encoded[i] ^ encoded[n - 2 - i];
            }
        }

        for i in 1..=m {
            perm[m - i] = encoded[m - i] ^ perm[m - i + 1];
            perm[m + i] = encoded[m + i - 1] ^ perm[m + i - 1];
        }

        perm
    }
}
```
