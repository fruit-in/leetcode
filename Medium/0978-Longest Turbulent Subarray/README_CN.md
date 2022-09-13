# 978. 最长湍流子数组
给定一个整数数组 `arr` ，返回 `arr` 的 *最大湍流子数组的**长度*** 。

如果比较符号在子数组中的每个相邻元素对之间翻转，则该子数组是 **湍流子数组** 。

更正式地来说，当 `arr` 的子数组 `A[i], A[i+1], ..., A[j]` 满足仅满足下列条件时，我们称其为湍流子数组：
* 若 `i <= k < j` ：
    * 当 `k` 为奇数时， `A[k] > A[k+1]`，且
    * 当 `k` 为偶数时，`A[k] < A[k+1]`；
* 或 若 `i <= k < j` ：
    * 当 `k` 为偶数时，`A[k] > A[k+1]` ，且
    * 当 `k` 为奇数时， `A[k] < A[k+1]`。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [9,4,2,10,7,8,8,1,9]
<strong>输出:</strong> 5
<strong>解释:</strong> arr[1] > arr[2] < arr[3] > arr[4] < arr[5]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [4,8,12,16]
<strong>输出:</strong> 2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [100]
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>1 <= arr.length <= 4 * 10<sup>4</sup></code>
* <code>0 <= arr[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let mut count0 = 1;
        let mut count1 = 1;
        let mut ret = 1;

        for k in 0..arr.len() - 1 {
            if (k % 2 == 1 && arr[k] > arr[k + 1]) || (k % 2 == 0 && arr[k] < arr[k + 1]) {
                count0 += 1;
            } else {
                ret = ret.max(count0);
                count0 = 1;
            }
            if (k % 2 == 0 && arr[k] > arr[k + 1]) || (k % 2 == 1 && arr[k] < arr[k + 1]) {
                count1 += 1;
            } else {
                ret = ret.max(count1);
                count1 = 1;
            }
        }

        ret.max(count0).max(count1)
    }
}
```
