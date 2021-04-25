# 1343. 大小为 K 且平均值大于等于阈值的子数组数目
给你一个整数数组 `arr` 和两个整数 `k` 和 `threshold` 。

请你返回长度为 `k` 且平均值大于等于 `threshold` 的子数组数目。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [2,2,2,2,5,5,5,8], k = 3, threshold = 4
<strong>输出:</strong> 3
<strong>解释:</strong> 子数组 [2,5,5],[5,5,5] 和 [5,5,8] 的平均值分别为 4，5 和 6 。其他长度为 3 的子数组的平均值都小于 4 （threshold 的值)。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,1,1,1,1], k = 1, threshold = 0
<strong>输出:</strong> 5
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [11,13,17,23,29,31,7,5,2,3], k = 3, threshold = 5
<strong>输出:</strong> 6
<strong>解释:</strong> 前 6 个长度为 3 的子数组平均值都大于 5 。注意平均值不是整数。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [7,7,7,7,7,7,7], k = 7, threshold = 7
<strong>输出:</strong> 1
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> arr = [4,4,4,4], k = 4, threshold = 1
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= arr.length <= 10^5`
* `1 <= arr[i] <= 10^4`
* `1 <= k <= arr.length`
* `0 <= threshold <= 10^4`

## 题解 (Ruby)

### 1. 滑动窗口
```Ruby
# @param {Integer[]} arr
# @param {Integer} k
# @param {Integer} threshold
# @return {Integer}
def num_of_subarrays(arr, k, threshold)
  sum = arr[...k].sum
  i = 0
  ret = sum >= k * threshold ? 1 : 0

  (k...arr.size).each do |j|
    sum += arr[j] - arr[i]
    i += 1
    ret += 1 if sum >= k * threshold
  end

  ret
end
```

## 题解 (Rust)

### 1. 滑动窗口
```Rust
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut sum = arr[..k as usize].iter().sum::<i32>();
        let mut i = 0;
        let mut ret = (sum >= k * threshold) as i32;

        for j in k as usize..arr.len() {
            sum += arr[j] - arr[i];
            i += 1;
            if sum >= k * threshold {
                ret += 1;
            }
        }

        ret
    }
}
```
