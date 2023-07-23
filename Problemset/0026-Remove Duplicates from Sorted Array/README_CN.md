# 26. 删除排序数组中的重复项
给定一个排序数组，你需要在**原地**删除重复出现的元素，使得每个元素只出现一次，返回移除后数组的新长度。

不要使用额外的数组空间，你必须在**原地修改输入数组**并在使用 O(1) 额外空间的条件下完成。

#### 示例 1:
<pre>
给定数组 <em>nums</em> = <strong>[1,1,2]</strong>,

函数应该返回新的长度 <strong>2</strong>, 并且原数组 <em>nums</em> 的前两个元素被修改为 <strong>1</strong>, <strong>2</strong>。

你不需要考虑数组中超出新长度后面的元素。
</pre>

#### 示例 2:
<pre>
给定 <em>nums</em> = <strong>[0,0,1,1,1,2,2,3,3,4]</strong>,

函数应该返回新的长度 <strong>5</strong>, 并且原数组 <em>nums</em> 的前五个元素被修改为 <strong>0</strong>, <strong>1</strong>, <strong>2</strong>, <strong>3</strong>, <strong>4</strong>。

你不需要考虑数组中超出新长度后面的元素。
</pre>

#### 说明:
为什么返回数值是整数，但输出的答案是数组呢?

请注意，输入数组是以**“引用”**方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。

你可以想象内部操作如下:
<pre>
// <strong>nums</strong> 是以“引用”方式传递的。也就是说，不对实参做任何拷贝
int len = removeDuplicates(nums);

// 在函数里修改输入数组对于调用者是可见的。
// 根据你的函数返回的长度, 它会打印出数组中<strong>该长度范围内</strong>的所有元素。
for (int i = 0; i < len; i++) {
    print(nums[i]);
}
</pre>

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0; }
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[i] != nums[j] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        i as i32 + 1
    }
}
```
