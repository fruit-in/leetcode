# 496. Next Greater Element I
You are given two arrays **(without duplicates)** ```nums1``` and ```nums2``` where ```nums1```’s elements are subset of ```nums2```. Find all the next greater numbers for ```nums1```'s elements in the corresponding places of ```nums2```.

The Next Greater Number of a number **x** in ```nums1``` is the first greater number to its right in ```nums2```. If it does not exist, output -1 for this number.

#### Example 1:
<pre>
<strong>Input:</strong> <strong>nums1</strong> = [4,1,2], <strong>nums2</strong> = [1,3,4,2].
<strong>Output:</strong> [-1,3,-1]
<strong>Explanation:</strong>
    For number 4 in the first array, you cannot find the next greater number for it in the second array, so output -1.
    For number 1 in the first array, the next greater number for it in the second array is 3.
    For number 2 in the first array, there is no next greater number for it in the second array, so output -1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> <strong>nums1</strong> = [2,4], <strong>nums2</strong> = [1,2,3,4].
<strong>Output:</strong> [3,-1]
<strong>Explanation:</strong>
    For number 2 in the first array, the next greater number for it in the second array is 3.
    For number 4 in the first array, there is no next greater number for it in the second array, so output -1.
</pre>

#### Note:
1. All elements in ```nums1``` and ```nums2``` are unique.
2. The length of both ```nums1``` and ```nums2``` would not exceed 1000.

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![-1; nums1.len()];

        for i in 0..nums1.len() {
            for j in (0..nums2.len()).rev() {
                if nums2[j] > nums1[i] {
                    ret[i] = nums2[j];
                } else if nums2[j] == nums1[i] {
                    break;
                }
            }
        }

        ret
    }
}
```

### 2. Stack & HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut num_greater = HashMap::new();

        for i in 0..nums2.len() {
            while !stack.is_empty() && *stack.last().unwrap() < nums2[i] {
                num_greater.insert(stack.pop().unwrap(), nums2[i]);
            }

            stack.push(nums2[i]);
        }

        nums1.iter().map(|x| *num_greater.get(x).unwrap_or(&-1)).collect()
    }
}
```
