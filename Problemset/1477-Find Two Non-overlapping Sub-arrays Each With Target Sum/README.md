# 1477. Find Two Non-overlapping Sub-arrays Each With Target Sum
You are given an array of integers `arr` and an integer `target`.

You have to find **two non-overlapping sub-arrays** of `arr` each with a sum equal `target`. There can be multiple answers so you have to find an answer where the sum of the lengths of the two sub-arrays is **minimum**.

Return *the minimum sum of the lengths* of the two required sub-arrays, or return `-1` if you cannot find such two sub-arrays.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [3,2,2,4,3], target = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> Only two sub-arrays have sum = 3 ([3] and [3]). The sum of their lengths is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [7,3,4,7], target = 7
<strong>Output:</strong> 2
<strong>Explanation:</strong> Although we have three non-overlapping sub-arrays of sum = 7 ([7], [3,4] and [7]), but we will choose the first and third sub-arrays as the sum of their lengths is 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [4,3,2,6,2,3,4], target = 6
<strong>Output:</strong> -1
<strong>Explanation:</strong> We have only one sub-array of sum = 6.
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* `1 <= arr[i] <= 1000`
* <code>1 <= target <= 10<sup>8</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut sum = 0;
        let mut pairs = vec![];
        let mut ret = -1;

        for j in 0..arr.len() {
            sum += arr[j];
            while i <= j && sum > target {
                sum -= arr[i];
                i += 1;
            }

            if sum == target {
                match pairs.binary_search(&(i as i32, -1)) {
                    Err(0) | Ok(_) => (),
                    Err(k) => {
                        let x = pairs[k - 1].0 - pairs[k - 1].1 + (j - i) as i32 + 2;
                        if ret == -1 || ret > x {
                            ret = x;
                        }
                    }
                }

                let (a, b) = *pairs.last().unwrap_or(&(i32::MAX, 0));
                if ((j - i) as i32) < a - b {
                    pairs.push((j as i32, i as i32));
                }
            }
        }

        ret
    }
}
```
