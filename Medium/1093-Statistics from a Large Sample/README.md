# 1093. Statistics from a Large Sample
We sampled integers between `0` and `255`, and stored the results in an array `count`:  `count[k]` is the number of integers we sampled equal to `k`.

Return the minimum, maximum, mean, median, and mode of the sample respectively, as an array of **floating point numbers**. The mode is guaranteed to be unique.

*(Recall that the median of a sample is:*
* *The middle element, if the elements of the sample were sorted and the number of elements is odd;*
* *The average of the middle two elements, if the elements of the sample were sorted and the number of elements is even.)*

#### Example 1:
<pre>
<strong>Input:</strong> count = [0,1,3,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
<strong>Output:</strong> [1.00000,3.00000,2.37500,2.50000,3.00000]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> count = [0,4,3,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
<strong>Output:</strong> [1.00000,4.00000,2.18182,2.00000,1.00000]
</pre>

#### Constraints:
1. `count.length == 256`
2. `1 <= sum(count) <= 10^9`
3. The mode of the sample that count represents is unique.
4. Answers within `10^-5` of the true value will be accepted as correct.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        vec![
            Self::minimum(&count),
            Self::maximum(&count),
            Self::mean(&count),
            Self::median(&count),
            Self::mode(&count),
        ]
    }

    pub fn minimum(count: &[i32]) -> f64 {
        count
            .iter()
            .enumerate()
            .filter(|(_, c)| **c > 0)
            .next()
            .unwrap()
            .0 as f64
    }

    pub fn maximum(count: &[i32]) -> f64 {
        count
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, c)| **c > 0)
            .next()
            .unwrap()
            .0 as f64
    }

    pub fn mean(count: &[i32]) -> f64 {
        let sum = count
            .iter()
            .enumerate()
            .map(|(k, c)| k as u64 * *c as u64)
            .sum::<u64>();
        let count_sum = count.iter().sum::<i32>();

        sum as f64 / count_sum as f64
    }

    pub fn median(count: &[i32]) -> f64 {
        let mut l = 0;
        let mut r = 255;
        let mut l_count = count[l];
        let mut r_count = count[r];

        while l < r {
            if l_count > r_count {
                l_count -= r_count;
                r -= 1;
                r_count = count[r];
            } else if l_count < r_count {
                r_count -= l_count;
                l += 1;
                l_count = count[l];
            } else {
                l += 1;
                l_count = count[l];
                r -= 1;
                r_count = count[r];
            }
        }

        (l + r) as f64 / 2.
    }

    pub fn mode(count: &[i32]) -> f64 {
        count.iter().enumerate().max_by_key(|(_, c)| *c).unwrap().0 as f64
    }
}
```
