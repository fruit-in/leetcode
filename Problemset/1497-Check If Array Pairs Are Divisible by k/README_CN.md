# 1497. 检查数组对是否可以被 k 整除
给你一个整数数组 `arr` 和一个整数 `k` ，其中数组长度是偶数，值为 `n` 。

现在需要把数组恰好分成 `n / 2` 对，以使每对数字的和都能够被 `k` 整除。

如果存在这样的分法，请返回 *True* ；否则，返回 *False* 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,2,3,4,5,10,6,7,8,9], k = 5
<strong>输出:</strong> true
<strong>解释:</strong> 划分后的数字对为 (1,9),(2,8),(3,7),(4,6) 以及 (5,10) 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,2,3,4,5,6], k = 7
<strong>输出:</strong> true
<strong>解释:</strong> 划分后的数字对为 (1,6),(2,5) 以及 (3,4) 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,2,3,4,5,6], k = 10
<strong>输出:</strong> false
<strong>解释:</strong> 无法在将数组中的数字分为三对的同时满足每对数字和能够被 10 整除的条件。
</pre>

#### 提示:
* `arr.length == n`
* <code>1 <= n <= 10<sup>5</sup></code>
* `n` 为偶数
* <code>-10<sup>9</sup> <= arr[i] <= 10<sup>9</sup></code>
* <code>1 <= k <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut count = vec![0; k as usize];

        for &x in &arr {
            count[x.rem_euclid(k) as usize] += 1;
        }

        for i in 1..(k as usize + 1) / 2 {
            if count[i] != count[k as usize - i] {
                return false;
            }
        }

        k % 2 == 1 || count[k as usize / 2] % 2 == 0
    }
}
```
