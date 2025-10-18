# 2640. 一个数组所有前缀的分数
定义一个数组 `arr` 的 **转换数组** `conver` 为：
* `conver[i] = arr[i] + max(arr[0..i])`，其中 `max(arr[0..i])` 是满足 `0 <= j <= i` 的所有 `arr[j]` 中的最大值。

定义一个数组 `arr` 的 **分数** 为 `arr` 转换数组中所有元素的和。

给你一个下标从 **0** 开始长度为 `n` 的整数数组 `nums` ，请你返回一个长度为 `n` 的数组 `ans` ，其中 `ans[i]`是前缀 `nums[0..i]` 的分数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,7,5,10]
<strong>输出:</strong> [4,10,24,36,56]
<strong>解释:</strong>
对于前缀 [2] ，转换数组为 [4] ，所以分数为 4 。
对于前缀 [2, 3] ，转换数组为 [4, 6] ，所以分数为 10 。
对于前缀 [2, 3, 7] ，转换数组为 [4, 6, 14] ，所以分数为 24 。
对于前缀 [2, 3, 7, 5] ，转换数组为 [4, 6, 14, 12] ，所以分数为 36 。
对于前缀 [2, 3, 7, 5, 10] ，转换数组为 [4, 6, 14, 12, 20] ，所以分数为 56 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,2,4,8,16]
<strong>输出:</strong> [2,4,8,16,32,64]
<strong>解释:</strong>
对于前缀 [1] ，转换数组为 [2] ，所以分数为 2 。
对于前缀 [1, 1]，转换数组为 [2, 2] ，所以分数为 4 。
对于前缀 [1, 1, 2]，转换数组为 [2, 2, 4] ，所以分数为 8 。
对于前缀 [1, 1, 2, 4]，转换数组为 [2, 2, 4, 8] ，所以分数为 16 。
对于前缀 [1, 1, 2, 4, 8]，转换数组为 [2, 2, 4, 8, 16] ，所以分数为 32 。
对于前缀 [1, 1, 2, 4, 8, 16]，转换数组为 [2, 2, 4, 8, 16, 32] ，所以分数为 64 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut max_num = 0;
        let mut conver = vec![0; n];
        let mut ans = vec![0; n];

        for i in 0..n {
            max_num = max_num.max(nums[i]);
            conver[i] = nums[i] + max_num;
            ans[i] = ans[i.max(1) - 1] + conver[i] as i64;
        }

        ans
    }
}
```
