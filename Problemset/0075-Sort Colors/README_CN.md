# 75. 颜色分类
给定一个包含红色、白色和蓝色，一共 *n* 个元素的数组，**原地**对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。

此题中，我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。

#### 注意:
不能使用代码库中的排序函数来解决这道题。

#### 示例:
<pre>
<strong>输入:</strong> [2,0,2,1,1,0]
<strong>输出:</strong> [0,0,1,1,2,2]
</pre>

#### 进阶:
* 一个直观的解决方案是使用计数排序的两趟扫描算法。<br>
首先，迭代计算出0、1 和 2 元素的个数，然后按照0、1、2的排序，重写当前数组。
* 你能想出一个仅使用常数空间的一趟扫描算法吗？

## 题解 (Rust)

### 1. 一次遍历
```Rust
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0usize;
        let mut right = nums.len() - 1;
        let mut index = 0usize;
        while index <= right {
            if nums[index] == 0 && index > left {
                while left < index && nums[left] == 0 {
                    left += 1;
                }
                nums.swap(left, index);
            } else if nums[index] == 2 && index < right {
                while right > index && nums[right] == 2 {
                    right -= 1;
                }
                nums.swap(index, right);
            } else {
                index += 1;
            }
        }
    }
}
```
