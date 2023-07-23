# 1566. 重复至少 K 次且长度为 M 的模式
给你一个正整数数组 `arr`，请你找出一个长度为 `m` 且在数组中至少重复 `k` 次的模式。

**模式** 是由一个或多个值组成的子数组（连续的子序列），**连续** 重复多次但 **不重叠** 。 模式由其长度和重复次数定义。

如果数组中存在至少重复 `k` 次且长度为 `m` 的模式，则返回 `true` ，否则返回  `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,2,4,4,4,4], m = 1, k = 3
<strong>输出:</strong> true
<strong>解释:</strong> 模式 <b>(4)</b> 的长度为 1 ，且连续重复 4 次。注意，模式可以重复 k 次或更多次，但不能少于 k 次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,2,1,2,1,1,1,3], m = 2, k = 2
<strong>输出:</strong> true
<strong>解释:</strong> 模式 <b>(1,2)</b> 长度为 2 ，且连续重复 2 次。另一个符合题意的模式是 <b>(2,1)</b> ，同样重复 2 次。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,2,1,2,1,3], m = 2, k = 3
<strong>输出:</strong> false
<strong>解释:</strong> 模式 <b>(1,2)</b> 长度为 2 ，但是只连续重复 2 次。不存在长度为 2 且至少重复 3 次的模式。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [1,2,3,1,2], m = 2, k = 2
<strong>输出:</strong> false
<strong>解释:</strong> 模式 <b>(1,2)</b> 出现 2 次但并不连续，所以不能算作连续重复 2 次。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> arr = [2,2,2,2], m = 2, k = 3
<strong>输出:</strong> false
<strong>解释:</strong> 长度为 2 的模式只有 <b>(2,2)</b> ，但是只连续重复 2 次。注意，不能计算重叠的重复次数。
</pre>

#### 提示:
* `2 <= arr.length <= 100`
* `1 <= arr[i] <= 100`
* `1 <= m <= 100`
* `2 <= k <= 100`

## 题解 (Rust)

### 1.暴力
```Rust
impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;

        if arr.len() < m * k {
            return false;
        }

        for i in 0..=(arr.len() - m * k) {
            if (i..(i + m * (k - 1))).all(|j| arr[j] == arr[j + m]) {
                return true;
            }
        }

        false
    }
}
```
