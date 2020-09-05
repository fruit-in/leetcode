# 1375. Bulb Switcher III
There is a room with `n` bulbs, numbered from `1` to `n`, arranged in a row from left to right. Initially, all the bulbs are turned off.

At moment *k* (for *k* from `0` to `n - 1`), we turn on the `light[k]` bulb. A bulb **change color to blue** only if it is on and all the previous bulbs (to the left) are turned on too.

Return the number of moments in which **all turned on** bulbs **are blue**.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/02/29/sample_2_1725.png)
<pre>
<strong>Input:</strong> light = [2,1,3,5,4]
<strong>Output:</strong> 3
<strong>Explanation:</strong> All bulbs turned on, are blue at the moment 1, 2 and 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> light = [3,2,4,1,5]
<strong>Output:</strong> 2
<strong>Explanation:</strong> All bulbs turned on, are blue at the moment 3, and 4 (index-0).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> light = [4,1,2,3]
<strong>Output:</strong> 1
<strong>Explanation:</strong> All bulbs turned on, are blue at the moment 3 (index-0).
Bulb 4th changes to blue at the moment 3.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> light = [2,1,4,3,6,5]
<strong>Output:</strong> 3
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> light = [1,2,3,4,5,6]
<strong>Output:</strong> 6
</pre>

#### Constraints:
* `n == light.length`
* `1 <= n <= 5 * 10^4`
* `light` is a permutation of  `[1, 2, ..., n]`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut ret = 0;

        for k in 0..light.len() {
            max = max.max(light[k]);
            if max == k as i32 + 1 {
                ret += 1;
            }
        }

        ret
    }
}
```
