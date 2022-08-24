# 1300. 转变数组后最接近目标值的数组和
给你一个整数数组 `arr` 和一个目标值 `target` ，请你返回一个整数 `value` ，使得将数组中所有大于 `value` 的值变成 `value` 后，数组的和最接近  `target` （最接近表示两者之差的绝对值最小）。

如果有多种使得和最接近 `target` 的方案，请你返回这些整数中的最小值。

请注意，答案不一定是 `arr` 中的数字。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [4,9,3], target = 10
<strong>输出:</strong> 3
<strong>解释:</strong> 当选择 value 为 3 时，数组会变成 [3, 3, 3]，和为 9 ，这是最接近 target 的方案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [2,3,5], target = 10
<strong>输出:</strong> 5
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [60864,25176,27249,21296,20204], target = 56803
<strong>输出:</strong> 11361
</pre>

#### 提示:
* <code>1 <= arr.length <= 10<sup>4</sup></code>
* <code>1 <= arr[i], target <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr = arr;
        let mut prefix_sum = 0;
        let mut diff = target;
        let mut value = 0;

        arr.push(0);
        arr.sort_unstable();

        for i in 1..arr.len() {
            let mut l = arr[i - 1];
            let mut r = arr[i];

            while l <= r {
                let m = (l + r) / 2;
                let sum = prefix_sum + m * (arr.len() - i) as i32;

                if sum == target {
                    return m;
                } else if sum < target {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }

            let mut rdiff = (prefix_sum + r * (arr.len() - i) as i32 - target).abs();
            let mut ldiff = (prefix_sum + l * (arr.len() - i) as i32 - target).abs();

            if r >= arr[i - 1] && rdiff < diff {
                diff = rdiff;
                value = r;
            }
            if l <= arr[i] && ldiff < diff {
                diff = ldiff;
                value = l;
            }

            prefix_sum += arr[i];
        }

        value
    }
}
```
