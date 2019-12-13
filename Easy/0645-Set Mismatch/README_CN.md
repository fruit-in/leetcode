# 645. 错误的集合
集合 ```S``` 包含从1到 ```n``` 的整数。不幸的是，因为数据错误，导致集合里面某一个元素复制了成了集合里面的另外一个元素的值，导致集合丢失了一个整数并且有一个元素重复。

给定一个数组 ```nums``` 代表了集合 ```S``` 发生错误后的结果。你的任务是首先寻找到重复出现的整数，再找到丢失的整数，将它们以数组的形式返回。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,2,4]
<strong>输出:</strong> [2,3]
</pre>

#### 注意:
1. 给定数组的长度范围是 [2, 10000]。
2. 给定的数组是无序的。

## 题解 (Rust)

### 1. 集合
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut set = (1..=(nums.len() as i32)).collect::<HashSet<i32>>();
        let mut dup = 0;
        let mut miss = 0;

        for num in nums {
            if !set.remove(&num) {
                dup = num;
            }
        }

        miss = set.drain().next().unwrap();

        vec![dup, miss]
    }
}
```

### 2. 排序
```Rust
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut dup = 0;
        let mut miss = nums.len() as i32;

        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                dup = nums[i];
            } else if nums[i - 1] == nums[i] - 2 {
                miss = nums[i] - 1;
            }
        }

        match nums[0] {
            1 => vec![dup, miss],
            _ => vec![dup, 1],
        }
    }
}
```

### 3. 标记位置
```Rust
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut dup = 0;
        let mut miss = 0;

        for i in 0..nums.len() {
            let j = nums[i].abs() as usize - 1;
            if nums[j] < 0 {
                dup = nums[i].abs();
            } else {
                nums[j] = -nums[j];
            }
        }

        for i in 0..nums.len() {
            if nums[i] > 0 {
                miss = i as i32 + 1;
                break;
            }
        }

        vec![dup, miss]
    }
}
```
