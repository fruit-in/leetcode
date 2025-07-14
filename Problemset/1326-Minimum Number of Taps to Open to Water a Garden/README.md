# 1326. Minimum Number of Taps to Open to Water a Garden
There is a one-dimensional garden on the x-axis. The garden starts at the point `0` and ends at the point `n`. (i.e., the length of the garden is `n`).

There are `n + 1` taps located at points `[0, 1, ..., n]` in the garden.

Given an integer `n` and an integer array `ranges` of length `n + 1` where `ranges[i]` (0-indexed) means the `i-th` tap can water the area `[i - ranges[i], i + ranges[i]]` if it was open.

Return *the minimum number of taps* that should be open to water the whole garden, If the garden cannot be watered return **-1**.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/01/16/1685_example_1.png)
<pre>
<strong>Input:</strong> n = 5, ranges = [3,4,1,1,0,0]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The tap at point 0 can cover the interval [-3,3]
The tap at point 1 can cover the interval [-3,5]
The tap at point 2 can cover the interval [1,3]
The tap at point 3 can cover the interval [2,4]
The tap at point 4 can cover the interval [4,4]
The tap at point 5 can cover the interval [5,5]
Opening Only the second tap will water the whole garden [0,5]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, ranges = [0,0,0,0]
<strong>Output:</strong> -1
<strong>Explanation:</strong> Even if you activate all the four taps you cannot water the whole garden.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>4</sup></code>
* `ranges.length == n + 1`
* `0 <= ranges[i] <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut new_ranges = vec![];
        let mut i = 0;
        let mut ret = 0;

        for (j, &r) in ranges.iter().enumerate() {
            let (start1, end1) = (j as i32 - r, j as i32 + r);
            let mut flag = true;

            while let Some(&(start2, end2)) = new_ranges.last() {
                if start1 <= start2 {
                    new_ranges.pop();
                    continue;
                }
                flag = end2 < end1;
                break;
            }

            if flag {
                new_ranges.push((start1, end1));
            }
        }

        for j in 0..new_ranges.len() {
            if i >= n {
                break;
            } else if new_ranges[j].0 > i {
                return -1;
            } else if j == new_ranges.len() - 1 || new_ranges[j + 1].0 > i {
                i = new_ranges[j].1;
                ret += 1;
            }
        }

        if i < n {
            return -1;
        }

        ret
    }
}
```
