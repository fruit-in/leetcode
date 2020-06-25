# 477. Total Hamming Distance
The [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) between two integers is the number of positions at which the corresponding bits are different.

Now your job is to find the total Hamming distance between all pairs of the given numbers.

#### Example:
<pre>
<strong>Input:</strong> 4, 14, 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> In binary representation, the 4 is 0100, 14 is 1110, and 2 is 0010 (just
showing the four bits relevant in this case). So the answer will be:
HammingDistance(4, 14) + HammingDistance(4, 2) + HammingDistance(14, 2) = 2 + 2 + 2 = 6.
</pre>

#### Note:
1. Elements of the given array are in the range of `0` to `10^9`
2. Length of the array will not exceed `10^4`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 0..30 {
            let mut zeros = 0;
            let mut ones = 0;
            for num in &nums {
                match (1 << i) & num {
                    0 => zeros += 1,
                    _ => ones += 1,
                }
            }
            ret += zeros * ones;
        }

        ret
    }
}
```
