# 1539. 第 k 个缺失的正整数
给你一个 **严格升序排列** 的正整数数组 `arr` 和一个整数 `k` 。

请你找到这个数组里第 `k` 个缺失的正整数。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [2,3,4,7,11], k = 5
<strong>输出:</strong> 9
<strong>解释:</strong> 缺失的正整数包括 [1,5,6,8,9,10,12,13,...] 。第 5 个缺失的正整数为 9 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,2,3,4], k = 2
<strong>输出:</strong> 6
<strong>解释:</strong> 缺失的正整数包括 [5,6,7,...] 。第 2 个缺失的正整数为 6 。
</pre>

#### 提示:
* `1 <= arr.length <= 1000`
* `1 <= arr[i] <= 1000`
* `1 <= k <= 1000`
* 对于所有 `1 <= i < j <= arr.length` 的 `i` 和 `j` 满足 `arr[i] < arr[j]`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let mut i = 0;

        for n in 1..2001 {
            if i >= arr.len() || n < arr[i] {
                k -= 1;
            } else {
                i += 1;
            }
            if k == 0 {
                return n;
            }
        }

        0
    }
}
```
