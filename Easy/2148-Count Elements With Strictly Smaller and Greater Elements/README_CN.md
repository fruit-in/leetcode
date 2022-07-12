# 2148. 元素计数
给你一个整数数组 `nums` ，统计并返回在 `nums` 中同时至少具有一个严格较小元素和一个严格较大元素的元素数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [11,7,2,15]
<strong>输出:</strong> 2
<strong>解释:</strong> 元素 7 ：严格较小元素是元素 2 ，严格较大元素是元素 11 。
元素 11 ：严格较小元素是元素 7 ，严格较大元素是元素 15 。
总计有 2 个元素都满足在 nums 中同时存在一个严格较小元素和一个严格较大元素。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-3,3,3,90]
<strong>输出:</strong> 2
<strong>解释:</strong> 元素 3 ：严格较小元素是元素 -3 ，严格较大元素是元素 90 。
由于有两个元素的值为 3 ，总计有 2 个元素都满足在 nums 中同时存在一个严格较小元素和一个严格较大元素。
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();

        nums.iter().filter(|&num| num != min && num != max).count() as i32
    }
}
```
