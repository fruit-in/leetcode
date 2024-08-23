# 1477. 找两个和为目标值且不重叠的子数组
给你一个整数数组 `arr` 和一个整数值 `target` 。

请你在 `arr` 中找 **两个互不重叠的子数组** 且它们的和都等于 `target` 。可能会有多种方案，请你返回满足要求的两个子数组长度和的 **最小值** 。

请返回满足要求的最小长度和，如果无法找到这样的两个子数组，请返回 **-1** 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [3,2,2,4,3], target = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 只有两个子数组和为 3 （[3] 和 [3]）。它们的长度和为 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [7,3,4,7], target = 7
<strong>输出:</strong> 2
<strong>解释:</strong> 尽管我们有 3 个互不重叠的子数组和为 7 （[7], [3,4] 和 [7]），但我们会选择第一个和第三个子数组，因为它们的长度和 2 是最小值。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [4,3,2,6,2,3,4], target = 6
<strong>输出:</strong> -1
<strong>解释:</strong> 我们只有一个和为 6 的子数组。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [5,5,4,4,5], target = 3
<strong>输出:</strong> -1
<strong>解释:</strong> 我们无法找到和为 3 的子数组。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> arr = [3,1,1,1,5,1,2,1], target = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 注意子数组 [1,2] 和 [2,1] 不能成为一个方案因为它们重叠了。
</pre>

#### 提示:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* `1 <= arr[i] <= 1000`
* <code>1 <= target <= 10<sup>8</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut sum = 0;
        let mut pairs = vec![];
        let mut ret = -1;

        for j in 0..arr.len() {
            sum += arr[j];
            while i <= j && sum > target {
                sum -= arr[i];
                i += 1;
            }

            if sum == target {
                match pairs.binary_search(&(i as i32, -1)) {
                    Err(0) | Ok(_) => (),
                    Err(k) => {
                        let x = pairs[k - 1].0 - pairs[k - 1].1 + (j - i) as i32 + 2;
                        if ret == -1 || ret > x {
                            ret = x;
                        }
                    }
                }

                let (a, b) = *pairs.last().unwrap_or(&(i32::MAX, 0));
                if ((j - i) as i32) < a - b {
                    pairs.push((j as i32, i as i32));
                }
            }
        }

        ret
    }
}
```
