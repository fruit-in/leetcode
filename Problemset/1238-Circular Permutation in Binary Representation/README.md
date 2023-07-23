# 1238. Circular Permutation in Binary Representation
Given 2 integers `n` and `start`. Your task is return **any** permutation `p` of `(0,1,2.....,2^n -1)` such that :
* `p[0] = start`
* `p[i]` and `p[i+1]` differ by only one bit in their binary representation.
* `p[0]` and `p[2^n -1]` must also differ by only one bit in their binary representation.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2, start = 3
<strong>Output:</strong> [3,2,0,1]
<strong>Explanation:</strong> The binary representation of the permutation is (11,10,00,01). 
All the adjacent element differ by one bit. Another valid permutation is [3,1,0,2]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, start = 2
<strong>Output:</strong> [2,6,7,5,4,0,1,3]
<strong>Explanation:</strong> The binary representation of the permutation is (010,110,111,101,100,000,001,011).
</pre>

#### Constraints:
* `1 <= n <= 16`
* `0 <= start < 2 ^ n`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut x = 1;
        let mut start_index = 0;
        let mut ret = vec![0];

        for _ in 0..n {
            let rev = ret.iter().rev().map(|&num| num + x).collect::<Vec<i32>>();
            for i in 0..rev.len() {
                ret.push(rev[i]);
                if rev[i] == start {
                    start_index = ret.len() - 1;
                }
            }
            x *= 2;
        }

        ret.rotate_left(start_index);

        ret
    }
}
```
