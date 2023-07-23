# 448. 找到所有数组中消失的数字
给定一个范围在  1 ≤ a[i] ≤ *n* ( *n* = 数组大小 ) 的 整型数组，数组中的元素一些出现了两次，另一些只出现一次。

找到所有在 [1, *n*] 范围之间没有出现在数组中的数字。

您能在不使用额外空间且时间复杂度为O(*n*)的情况下完成这个任务吗? 你可以假定返回的数组不算在额外空间内。

#### 示例:
<pre>
<strong>输入:</strong>
[4,3,2,7,8,2,3,1]
<strong>输出:</strong>
[5,6]
</pre>

## 题解 (Rust)

### 1. 交换位置
```Rust
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let mut curr = nums[i];

            while nums[curr as usize - 1] != curr {
                let next = nums[curr as usize - 1];
                nums[curr as usize - 1] = curr;
                curr = next;
            }
        }

        for i in 0..nums.len() {
            if i + 1 != nums[i] as usize {
                ret.push(i as i32 + 1);
            }
        }

        ret
    }
}
```

### 2. 标记位置
```Rust
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let j = nums[i].abs() as usize - 1;
            nums[j] = -nums[j].abs();
        }

        for i in 0..nums.len() {
            if nums[i] > 0 {
                ret.push(i as i32 + 1);
            }
        }

        ret
    }
}
```
