# 581. Shortest Unsorted Continuous Subarray
Given an integer array, you need to find one **continuous subarray** that if you only sort this subarray in ascending order, then the whole array will be sorted in ascending order, too.

You need to find the **shortest** such subarray and output its length.

#### Example 1:
<pre>
<strong>Input:</strong> [2, 6, 4, 8, 10, 9, 15]
<strong>Output:</strong> 5
<strong>Explanation:</strong> You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.
</pre>

#### Note:
1. Then length of the input array is in range [1, 10,000].
2. The input array may contain duplicates, so ascending order here means **<=**. 

## Solutions (Rust)

### 1. Brute Force
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

### 2. Sort
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

### 3. Store Min & Max
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

### 4. Stack
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
