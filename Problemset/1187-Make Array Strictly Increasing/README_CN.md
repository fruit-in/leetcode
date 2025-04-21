# 1187. 使数组严格递增
给你两个整数数组 `arr1` 和 `arr2`，返回使 `arr1` 严格递增所需要的最小「操作」数（可能为 0）。

每一步「操作」中，你可以分别从 `arr1` 和 `arr2` 中各选出一个索引，分别为 `i` 和 `j`，`0 <= i < arr1.length` 和 `0 <= j < arr2.length`，然后进行赋值运算 `arr1[i] = arr2[j]`。

如果无法让 `arr1` 严格递增，请返回 `-1`。

#### 示例 1:
<pre>
<strong>输入:</strong> arr1 = [1,5,3,6,7], arr2 = [1,3,2,4]
<strong>输出:</strong> 1
<strong>解释:</strong> 用 2 来替换 5，之后 arr1 = [1, 2, 3, 6, 7]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr1 = [1,5,3,6,7], arr2 = [4,3,1]
<strong>输出:</strong> 2
<strong>解释:</strong> 用 3 来替换 5，然后用 4 来替换 3，得到 arr1 = [1, 3, 4, 6, 7]。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr1 = [1,5,3,6,7], arr2 = [1,6,3,3]
<strong>输出:</strong> -1
<strong>解释:</strong> 无法使 arr1 严格递增。
</pre>

#### 提示:
* `1 <= arr1.length, arr2.length <= 2000`
* `0 <= arr1[i], arr2[i] <= 10^9`

## 题解 (Rust)

### 1. 题解
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
