# 167. Two Sum II - Input array is sorted
Given an array of integers that is already ***sorted in ascending order***, find two numbers such that they add up to a specific target number.

The function twoSum should return indices of the two numbers such that they add up to the target, where index1 must be less than index2.

#### Note:
* Your returned answers (both index1 and index2) are not zero-based.
* You may assume that each input would have exactly one solution and you may not use the same element twice.

#### Example:
<pre>
<strong>Input:</strong> numbers = [2,7,11,15], target = 9
<strong>Output:</strong> [1, 2]
<strong>Explanation:</strong> The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.
</pre>

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let len = numbers.len();
        for i in 0..len {
            let tg = target - numbers[i];
            let mut left = i + 1;
            let mut right = len - 1;
            let mut mid = (left + right) / 2;
            while left <= right {
                if numbers[mid] == tg {
                    return vec![i as i32 + 1, mid as i32 + 1];
                } else if numbers[mid] < tg {
                    left = mid + 1;
                    mid = (left + right) / 2;
                } else {
                    right = mid - 1;
                    mid = (left + right) / 2;
                }
            }
        }
        Vec::new()
    }
}
```

### 2. Two Pointers
```Rust
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while numbers[i] + numbers[j] != target {
            if numbers[i] + numbers[j] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        vec![i as i32 + 1, j as i32 + 1]
    }
}
```
