# 907. 子数组的最小值之和
给定一个整数数组 `arr`，找到 `min(b)` 的总和，其中 `b` 的范围为 `arr` 的每个（连续）子数组。

由于答案可能很大，因此 **返回答案模** `10^9 + 7` 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [3,1,2,4]
<strong>输出:</strong> 17
<strong>解释:</strong>
子数组为 [3]，[1]，[2]，[4]，[3,1]，[1,2]，[2,4]，[3,1,2]，[1,2,4]，[3,1,2,4]。
最小值为 3，1，2，4，1，1，2，1，1，1，和为 17。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [11,81,94,43,3]
<strong>输出:</strong> 444
</pre>

#### 提示:
* <code>1 <= arr.length <= 3 * 10<sup>4</sup></code>
* <code>1 <= arr[i] <= 3 * 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut less = vec![(0, 0); arr.len()];
        let mut ret = 0;

        for i in 0..arr.len() {
            while stack.last().unwrap_or(&(0, 0)).1 >= arr[i] {
                stack.pop();
            }

            less[i].0 = i as i64 - stack.last().unwrap_or(&(-1, 0)).0;
            stack.push((i as i64, arr[i]));
        }

        stack.clear();

        for i in (0..arr.len()).rev() {
            while stack.last().unwrap_or(&(0, 0)).1 > arr[i] {
                stack.pop();
            }

            less[i].1 = stack.last().unwrap_or(&(arr.len() as i64, 0)).0 - i as i64;
            ret = (ret + arr[i] as i64 * less[i].0 * less[i].1) % 1_000_000_007;
            stack.push((i as i64, arr[i]));
        }

        ret as i32
    }
}
```
