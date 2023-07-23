# 2145. Count the Hidden Sequences
You are given a **0-indexed** array of `n` integers `differences`, which describes the **differences** between each pair of **consecutive** integers of a **hidden** sequence of length `(n + 1)`. More formally, call the hidden sequence `hidden`, then we have that `differences[i] = hidden[i + 1] - hidden[i]`.

You are further given two integers `lower` and `upper` that describe the **inclusive** range of values `[lower, upper]` that the hidden sequence can contain.

* For example, given `differences = [1, -3, 4]`, `lower = 1`, `upper = 6`, the hidden sequence is a sequence of length 4 whose elements are in between `1` and `6` (**inclusive**).
  * `[3, 4, 1, 5]` and `[4, 5, 2, 6]` are possible hidden sequences.
  * `[5, 6, 3, 7]` is not possible since it contains an element greater than `6`.
  * `[1, 2, 3, 4]` is not possible since the differences are not correct.

Return *the number of **possible** hidden sequences there are*. If there are no possible sequences, return `0`.

#### Example 1:
<pre>
<strong>Input:</strong> differences = [1,-3,4], lower = 1, upper = 6
<strong>Output:</strong> 2
<strong>Explanation:</strong> The possible hidden sequences are:
- [3, 4, 1, 5]
- [4, 5, 2, 6]
Thus, we return 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> differences = [3,-4,5,1,-2], lower = -4, upper = 5
<strong>Output:</strong> 4
<strong>Explanation:</strong> The possible hidden sequences are:
- [-3, 0, -4, 1, 2, 0]
- [-2, 1, -3, 2, 3, 1]
- [-1, 2, -2, 3, 4, 2]
- [0, 3, -1, 4, 5, 3]
Thus, we return 4.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> differences = [4,-7,2], lower = 3, upper = 6
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no possible hidden sequences. Thus, we return 0.
</pre>

#### Constraints:
* `n == differences.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= differences[i] <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= lower <= upper <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut prefix_sum = 0;
        let mut max_num = 0;
        let mut min_num = 0;

        for &x in &differences {
            prefix_sum += x as i64;
            max_num = max_num.max(prefix_sum);
            min_num = min_num.min(prefix_sum);
        }

        ((upper - lower) as i64 + min_num - max_num + 1).max(0) as i32
    }
}
```
