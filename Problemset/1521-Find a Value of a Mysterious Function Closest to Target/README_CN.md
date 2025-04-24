# 1521. 找到最接近目标值的函数值
![](https://assets.leetcode.com/uploads/2020/07/09/change.png)

Winston 构造了一个如上所示的函数 `func` 。他有一个整数数组 `arr` 和一个整数 `target` ，他想找到让 `|func(arr, l, r) - target|` 最小的 `l` 和 `r` 。

请你返回 `|func(arr, l, r) - target|` 的最小值。

请注意， `func` 的输入参数 `l` 和 `r` 需要满足 `0 <= l, r < arr.length` 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [9,12,3,7,15], target = 5
<strong>输出:</strong> 2
<strong>解释:</strong> 所有可能的 [l,r] 数对包括 [[0,0],[1,1],[2,2],[3,3],[4,4],[0,1],[1,2],[2,3],[3,4],[0,2],[1,3],[2,4],[0,3],[1,4],[0,4]]， Winston 得到的相应结果为 [9,12,3,7,15,8,0,3,7,0,0,3,0,0,0] 。最接近 5 的值是 7 和 3，所以最小差值为 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1000000,1000000,1000000], target = 1
<strong>输出:</strong> 999999
<strong>解释:</strong> Winston 输入函数的所有可能 [l,r] 数对得到的函数值都为 1000000 ，所以最小差值为 999999 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,2,4,8,16], target = 0
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>1 <= arr[i] <= 10<sup>6</sup></code>
* <code>0 <= target <= 10<sup>7</sup></code>

## 题解 (Rust)

### 1. 题解
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
