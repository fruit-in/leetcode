# 2540. 最小公共值
给你两个整数数组 `nums1` 和 `nums2` ，它们已经按非降序排序，请你返回两个数组的 **最小公共整数** 。如果两个数组 `nums1` 和 `nums2` 没有公共整数，请你返回 `-1` 。

如果一个整数在两个数组中都 **至少出现一次** ，那么这个整数是数组 `nums1` 和 `nums2` **公共** 的。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,2,3], nums2 = [2,4]
<strong>输出:</strong> 2
<strong>解释:</strong> 两个数组的最小公共元素是 2 ，所以我们返回 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [1,2,3,6], nums2 = [2,3,4,5]
<strong>输出:</strong> 2
<strong>解释:</strong> 两个数组中的公共元素是 2 和 3 ，2 是较小值，所以返回 2 。
</pre>

#### 提示:
* <code>1 <= nums1.length, nums2.length <= 10<sup>5</sup></code>
* <code>1 <= nums1[i], nums2[j] <= 10<sup>9</sup></code>
* `nums1` 和 `nums2` 都是 **非降序** 的。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                return nums1[i];
            } else if nums1[i] < nums2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }

        -1
    }
}
```
