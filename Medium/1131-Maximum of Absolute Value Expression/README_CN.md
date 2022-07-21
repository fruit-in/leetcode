# 1131. 绝对值表达式的最大值
给你两个长度相等的整数数组，返回下面表达式的最大值：

`|arr1[i] - arr1[j]| + |arr2[i] - arr2[j]| + |i - j|`

其中下标 `i`，`j` 满足 `0 <= i, j < arr1.length`。

#### 示例 1:
<pre>
<strong>输入:</strong> arr1 = [1,2,3,4], arr2 = [-1,4,5,6]
<strong>输出:</strong> 13
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr1 = [1,-2,-5,0,10], arr2 = [0,-2,-1,-7,-4]
<strong>输出:</strong> 20
</pre>

#### 提示:
* `2 <= arr1.length == arr2.length <= 40000`
* `-10^6 <= arr1[i], arr2[i] <= 10^6`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut max = [i32::MIN; 4];
        let mut min = [i32::MAX; 4];

        for i in 0..arr1.len() {
            max[0] = max[0].max(i as i32 + arr1[i] + arr2[i]);
            max[1] = max[1].max(i as i32 + arr1[i] - arr2[i]);
            max[2] = max[2].max(i as i32 - arr1[i] + arr2[i]);
            max[3] = max[3].max(i as i32 - arr1[i] - arr2[i]);
            min[0] = min[0].min(i as i32 + arr1[i] + arr2[i]);
            min[1] = min[1].min(i as i32 + arr1[i] - arr2[i]);
            min[2] = min[2].min(i as i32 - arr1[i] + arr2[i]);
            min[3] = min[3].min(i as i32 - arr1[i] - arr2[i]);
        }

        (0..4).map(|i| max[i] - min[i]).max().unwrap()
    }
}
```
