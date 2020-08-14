# 27. 移除元素
给定一个数组 *nums* 和一个值 *val*，你需要**原地**移除所有数值等于 *val* 的元素，返回移除后数组的新长度。

不要使用额外的数组空间，你必须在**原地修改输入数组**并在使用 O(1) 额外空间的条件下完成。

元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。

#### 示例 1:
<pre>
给定 <em>nums</em> = <strong>[3,2,2,3]</strong>, <em>val</em> = <strong>3</strong>,

函数应该返回新的长度 <strong>2</strong>, 并且 <em>nums</em> 中的前两个元素均为 <strong>2</strong>。

你不需要考虑数组中超出新长度后面的元素。
</pre>

#### 示例 2:
<pre>
给定 <em>nums</em> = <strong>[0,1,2,2,3,0,4,2]</strong>, <em>val</em> = <strong>2</strong>,

函数应该返回新的长度 <strong>5</strong>, 并且 <em>nums</em> 中的前五个元素为 <strong>0</strong>, <strong>1</strong>, <strong>3</strong>, <strong>0</strong>, <strong>4</strong>。

注意这五个元素可为任意顺序。

你不需要考虑数组中超出新长度后面的元素。
</pre>

#### 说明:
为什么返回数值是整数，但输出的答案是数组呢?

请注意，输入数组是以**“引用”**方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。

你可以想象内部操作如下:
<pre>
// <strong>nums</strong> 是以“引用”方式传递的。也就是说，不对实参作任何拷贝
int len = removeElement(nums, val);

// 在函数里修改输入数组对于调用者是可见的。
// 根据你的函数返回的长度, 它会打印出数组中<strong>该长度范围内</strong>的所有元素。
for (int i = 0; i < len; i++) {
    print(nums[i]);
}
</pre>

## 题解 (Ruby)

### 1. 双指针
```Ruby
# @param {Integer[]} nums
# @param {Integer} val
# @return {Integer}
def remove_element(nums, val)
    ret = 0

    for i in 0...nums.length
        if nums[i] != val
            nums[ret] = nums[i]
            ret += 1
        end
    end

    return ret
end
```

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[ans] = nums[i];
                ans += 1;
            }
        }
        ans as i32
    }
}
```
