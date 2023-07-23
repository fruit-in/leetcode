# 80. 删除排序数组中的重复项 II
给定一个排序数组，你需要在<b>[原地](https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95)</b>删除重复出现的元素，使得每个元素最多出现两次，返回移除后数组的新长度。

不要使用额外的数组空间，你必须在<b>[原地](https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95)修改输入数组</b>并在使用 O(1) 额外空间的条件下完成。

#### 示例 1:
<pre>
给定 <i>nums</i> = <strong>[1,1,1,2,2,3]</strong>,

函数应返回新长度 length = <b>5</b>, 并且原数组的前五个元素被修改为 <b>1, 1, 2, 2, 3</b> 。

你不需要考虑数组中超出新长度后面的元素。
</pre>

#### 示例 2:
<pre>
给定 <i>nums</i> = <b>[0,0,1,1,1,1,2,3,3]</b>,

函数应返回新长度 length = <b>7</b>, 并且原数组的前五个元素被修改为 <b>0, 0, 1, 1, 2, 3, 3</b> 。

你不需要考虑数组中超出新长度后面的元素。
</pre>

#### 说明:
为什么返回数值是整数，但输出的答案是数组呢?

请注意，输入数组是以**“引用”**方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。

你可以想象内部操作如下:
<pre>
// <b>nums</b> 是以“引用”方式传递的。也就是说，不对实参做任何拷贝
int len = removeDuplicates(nums);

// 在函数里修改输入数组对于调用者是可见的。
// 根据你的函数返回的长度, 它会打印出数组中<b>该长度范围内</b>的所有元素。
for (int i = 0; i < len; i++) {
    print(nums[i]);
}
</pre>

## 题解 (Rust)

### 1. 删除重复项
```Rust
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 2;

        while i < nums.len() {
            if nums[i] == nums[i - 2] {
                nums.remove(i);
            } else {
                i += 1;
            }
        }

        nums.len() as i32
    }
}
```

### 2. 双指针
```Rust
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;

        for j in 0..nums.len() {
            if i < 2 || nums[j] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }

        i as i32
    }
}
```
