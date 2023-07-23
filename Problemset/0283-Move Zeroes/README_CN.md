# 283. 移动零
给定一个数组 ```nums```，编写一个函数将所有 ```0``` 移动到数组的末尾，同时保持非零元素的相对顺序。

#### 示例:
<pre>
<strong>输入:</strong> [0,1,0,3,12]
<strong>输出:</strong> [1,3,12,0,0]
</pre>

#### 说明:
1. 必须在原数组上操作，不能拷贝额外的数组。
2. 尽量减少操作次数。

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
        }
        for j in i..nums.len() {
            nums[j] = 0;
        }
    }
}
```

### 2. 单指针
```Rust
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for _ in 0..nums.len() {
            if nums[i] == 0 {
                nums.remove(i);
                nums.push(0);
            } else {
                i += 1;
            }
        }
    }
}
```
