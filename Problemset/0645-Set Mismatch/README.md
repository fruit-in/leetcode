# 645. Set Mismatch
The set ```S``` originally contains numbers from 1 to ```n```. But unfortunately, due to the data error, one of the numbers in the set got duplicated to **another** number in the set, which results in repetition of one number and loss of another number.

Given an array ```nums``` representing the data status of this set after the error. Your task is to firstly find the number occurs twice and then find the number that is missing. Return them in the form of an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,2,4]
<strong>Output:</strong> [2,3]
</pre>

#### Note:
1. The given array size will in the range [2, 10000].
2. The given array's numbers won't have any order.

## Solutions (Rust)

### 1. Set
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

### 2. Sort
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

### 3. Mark Positions
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
