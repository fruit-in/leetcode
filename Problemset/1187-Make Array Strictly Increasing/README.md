# 1187. Make Array Strictly Increasing
Given two integer arrays `arr1` and `arr2`, return the minimum number of operations (possibly zero) needed to make `arr1` strictly increasing.

In one operation, you can choose two indices `0 <= i < arr1.length` and `0 <= j < arr2.length` and do the assignment `arr1[i] = arr2[j]`.

If there is no way to make `arr1` strictly increasing, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> arr1 = [1,5,3,6,7], arr2 = [1,3,2,4]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Replace 5 with 2, then arr1 = [1, 2, 3, 6, 7].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr1 = [1,5,3,6,7], arr2 = [4,3,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Replace 5 with 3 and then replace 3 with 4. arr1 = [1, 3, 4, 6, 7].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr1 = [1,5,3,6,7], arr2 = [1,6,3,3]
<strong>Output:</strong> -1
<strong>Explanation:</strong> You can't make arr1 strictly increasing.
</pre>

#### Constraints:
* `1 <= arr1.length, arr2.length <= 2000`
* `0 <= arr1[i], arr2[i] <= 10^9`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        arr2.sort_unstable();
        arr2.dedup();

        let mut dp = vec![vec![i32::MAX; arr2.len() + 1]; arr1.len()];
        dp[0][arr2.len()] = 0;
        if arr2[0] < arr1[0] {
            dp[0][0] = 1;
        }

        for i in 1..arr1.len() {
            if arr1[i] > arr1[i - 1] {
                dp[i][arr2.len()] = dp[i - 1][arr2.len()];
            }

            for j in 0..arr2.len() {
                if arr2[j] < arr1[i] {
                    dp[i][arr2.len()] = dp[i][arr2.len()].min(dp[i - 1][j]);
                }
                if arr2[j] > arr1[i - 1] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][arr2.len()].saturating_add(1));
                }
                if j < arr2.len() - 1 {
                    dp[i][j + 1] = dp[i][j + 1].min(dp[i - 1][j].saturating_add(1));
                }
            }
        }

        *dp[arr1.len() - 1]
            .iter()
            .filter(|&&x| x != i32::MAX)
            .min()
            .unwrap_or(&-1)
    }
}
```
