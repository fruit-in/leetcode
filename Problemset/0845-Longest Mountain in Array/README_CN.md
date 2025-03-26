# 845. 数组中的最长山脉
把符合下列属性的数组 `arr` 称为 **山脉数组** ：
* `arr.length >= 3`
* 存在下标 `i`（`0 < i < arr.length - 1`），满足
    * `arr[0] < arr[1] < ... < arr[i - 1] < arr[i]`
    * `arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`

给出一个整数数组 `arr`，返回最长山脉子数组的长度。如果不存在山脉子数组，返回 `0` 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [2,1,4,7,3,2,5]
<strong>输出:</strong> 5
<strong>解释:</strong> 最长的山脉子数组是 [1,4,7,3,2]，长度为 5。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [2,2,2]
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在山脉子数组。
</pre>

#### 提示:
* <code>1 <= arr.length <= 10<sup>4</sup></code>
* <code>0 <= arr[i] <= 10<sup>4</sup></code>

#### 进阶:
* 你可以仅用一趟扫描解决此问题吗？
* 你可以用 `O(1)` 空间解决此问题吗？

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let mut upcount = 0;
        let mut downcount = 0;
        let mut ret = 0;

        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] {
                upcount = 0;
                downcount = 0;
            } else if arr[i] > arr[i - 1] && i > 1 && arr[i - 1] < arr[i - 2] {
                upcount = 1;
                downcount = 0;
            } else if arr[i] > arr[i - 1] {
                upcount += 1;
            } else {
                downcount += 1;
            }

            if upcount > 0 && downcount > 0 {
                ret = ret.max(upcount + downcount + 1);
            }
        }

        ret
    }
}
```
