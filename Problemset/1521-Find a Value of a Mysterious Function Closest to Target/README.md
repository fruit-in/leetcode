# 1521. Find a Value of a Mysterious Function Closest to Target
![](https://assets.leetcode.com/uploads/2020/07/09/change.png)

Winston was given the above mysterious function `func`. He has an integer array `arr` and an integer `target` and he wants to find the values `l` and `r` that make the value `|func(arr, l, r) - target|` minimum possible.

Return *the minimum possible value* of `|func(arr, l, r) - target|`.

Notice that `func` should be called with the values `l` and `r` where `0 <= l, r < arr.length`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [9,12,3,7,15], target = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> Calling func with all the pairs of [l,r] = [[0,0],[1,1],[2,2],[3,3],[4,4],[0,1],[1,2],[2,3],[3,4],[0,2],[1,3],[2,4],[0,3],[1,4],[0,4]], Winston got the following results [9,12,3,7,15,8,0,3,7,0,0,3,0,0,0]. The value closest to 5 is 7 and 3, thus the minimum difference is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1000000,1000000,1000000], target = 1
<strong>Output:</strong> 999999
<strong>Explanation:</strong> Winston called the func with all possible values of [l,r] and he always got 1000000, thus the min difference is 999999.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,2,4,8,16], target = 0
<strong>Output:</strong> 0
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>1 <= arr[i] <= 10<sup>6</sup></code>
* <code>0 <= target <= 10<sup>7</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let n = (*arr.iter().max().unwrap() as f64).log2().ceil() as usize;
        let mut prefix_zeros = vec![vec![0; n]; arr.len() + 1];
        let mut ret = i32::MAX;

        for i in 0..arr.len() {
            for j in 0..n {
                prefix_zeros[i + 1][j] = prefix_zeros[i][j] + 1 - ((arr[i] >> j) & 1);
            }
        }

        for r in 0..arr.len() {
            if arr[r] <= target {
                ret = ret.min(target - arr[r]);
                continue;
            }

            let mut x = 0;
            let mut lo = 0;
            let mut hi = r;

            while lo < hi {
                let mid = (lo + hi) / 2;

                x = (0..n)
                    .filter(|&i| prefix_zeros[r + 1][i] - prefix_zeros[mid][i] == 0)
                    .fold(0, |acc, i| acc | (1 << i));

                if x == target {
                    return 0;
                } else if x < target {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }

            x = (0..n)
                .filter(|&i| prefix_zeros[r + 1][i] - prefix_zeros[hi][i] == 0)
                .fold(0, |acc, i| acc | (1 << i));
            ret = ret.min((x - target).abs());

            if hi > 0 {
                x = (0..n)
                    .filter(|&i| prefix_zeros[r + 1][i] - prefix_zeros[hi - 1][i] == 0)
                    .fold(0, |acc, i| acc | (1 << i));
                ret = ret.min((x - target).abs());
            }
        }

        ret
    }
}
```
