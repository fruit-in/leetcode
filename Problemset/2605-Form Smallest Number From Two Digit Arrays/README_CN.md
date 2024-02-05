# 2605. 从两个数字数组里生成最小数字
给你两个只包含 1 到 9 之间数字的数组 `nums1` 和 `nums2` ，每个数组中的元素 **互不相同** ，请你返回 **最小** 的数字，两个数组都 **至少** 包含这个数字的某个数位。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [4,1,3], nums2 = [5,7]
<strong>输出:</strong> 15
<strong>解释:</strong> 数字 15 的数位 1 在 nums1 中出现，数位 5 在 nums2 中出现。15 是我们能得到的最小数字。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [3,5,2,6], nums2 = [3,1,7]
<strong>输出:</strong> 3
<strong>解释:</strong> 数字 3 的数位 3 在两个数组中都出现了。
</pre>

#### 提示:
* `1 <= nums1.length, nums2.length <= 9`
* `1 <= nums1[i], nums2[i] <= 9`
* 每个数组中，元素 **互不相同** 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut min0 = 10;
        let min1 = *nums1.iter().min().unwrap();
        let min2 = *nums2.iter().min().unwrap();

        for x in &nums1 {
            for y in &nums2 {
                if x == y {
                    min0 = min0.min(*x);
                }
            }
        }

        if min0 < 10 {
            min0
        } else if min1 < min2 {
            min1 * 10 + min2
        } else {
            min2 * 10 + min1
        }
    }
}
```
