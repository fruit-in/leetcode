# 581. 最短无序连续子数组
给定一个整数数组，你需要寻找一个**连续的子数组**，如果对这个子数组进行升序排序，那么整个数组都会变为升序排序。

你找到的子数组应是**最短**的，请输出它的长度。

#### 示例 1:
<pre>
<strong>输入:</strong> [2, 6, 4, 8, 10, 9, 15]
<strong>输出:</strong> 5
<strong>解释:</strong> 你只需要对 [6, 4, 8, 10, 9] 进行升序排序，那么整个表都会变为升序排序。
</pre>

#### 说明:
1. 输入的数组长度范围在 [1, 10,000]。
2. 输入的数组可能包含**重复**元素 ，所以**升序**的意思是<=。

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut l = nums.len();
        let mut r = 0;

        for i in 0..nums.len() {
            if nums.iter().skip(i + 1).any(|&x| x < nums[i]) {
                l = i;
                break;
            }
        }
        for i in (0..nums.len()).rev() {
            if nums.iter().take(i).any(|&x| x > nums[i]) {
                r = i;
                break;
            }
        }

        (r as i32 - l as i32 + 1).max(0)
    }
}
```

### 2. 排序
```Rust
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();

        let mut l = nums.len();
        let mut r = 0;

        for i in 0..nums.len() {
            if nums[i] != sorted_nums[i] {
                l = i;
                break;
            }
        }
        for i in (0..nums.len()).rev() {
            if nums[i] != sorted_nums[i] {
                r = i;
                break;
            }
        }

        (r as i32 - l as i32 + 1).max(0)
    }
}
```

### 3. 存储最值
```Rust
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;

        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                min = *nums.iter().skip(i).min().unwrap();
                break;
            }
        }

        for i in (0..(nums.len() - 1)).rev() {
            if nums[i] > nums[i + 1] {
                max = *nums.iter().take(i + 1).max().unwrap();
                break;
            }
        }

        let l = nums.iter().position(|&x| x > min).unwrap_or(nums.len());
        let r = nums.iter().rposition(|&x| x < max).unwrap_or(0);

        (r as i32 - l as i32 + 1).max(0)
    }
}
```

### 4. 栈
```Rust
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut l = nums.len();
        let mut r = 0;

        for i in 0..nums.len() {
            if !stack.is_empty() && *stack.last().unwrap() > nums[i] {
                while !stack.is_empty() && *stack.last().unwrap() > nums[i] {
                    stack.pop();
                }
                l = l.min(stack.len());
            }
            stack.push(nums[i]);
        }

        stack.clear();

        for i in (0..nums.len()).rev() {
            if !stack.is_empty() && *stack.last().unwrap() < nums[i] {
                while !stack.is_empty() && *stack.last().unwrap() < nums[i] {
                    stack.pop();
                }
                r = r.max(nums.len() - stack.len() - 1);
            }
            stack.push(nums[i]);
        }

        (r as i32 - l as i32 + 1).max(0)
    }
}
```
