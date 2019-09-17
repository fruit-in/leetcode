# 1. Two Sum
Given an array of integers, return **indices** of the two numbers such that they add up to a specific target.

You may assume that each input would have ***exactly*** one solution, and you may not use the same element twice.

#### Example:
<pre>
Given nums = [2, 7, 11, 15], target = 9,

Because nums[<strong>0</strong>] + nums[<strong>1</strong>] = 2 + 7 = 9,
return [<strong>0</strong>, <strong>1</strong>].
</pre>

## Solutions (Python)

### 1. Brute Force
```Python3
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for k, v in enumerate(nums):
            if target - v in nums[k + 1:]:
                return [k, k + 1 + nums[k + 1:].index(target - v)]
```

### 2. One Pass Hash Table
```Python3
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        s = {}
        for k, v in enumerate(nums):
            if target - v in s.keys():
                return [k, s[target - v]]
            s[v] = k
```

### 3. Two Pass Hash Table
```Python3
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        s = {}
        for k, v in enumerate(nums):
            s[v] = k
        for k, v in enumerate(nums):
            if target - v in s.keys() and s[target - v] != k:
                return [k, s[target - v]]
```

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        Vec::new()
    }
}
```

### 2. One Pass Hash Table
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            match map.get(&nums[i]) {
                Some(&j) => return vec![j as i32, i as i32],
                None => map.insert(target - nums[i], i),
            };
        }
        Vec::new()
    }
}
```

### 3. Two Pass Hash Table
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            map.insert(target - nums[i], i);
        }
        for i in 0..nums.len() {
            if let Some(&j) = map.get(&nums[i]) {
                if i != j {
                    return vec![i as i32, j as i32];
                }
            }
        }
        Vec::new()
    }
}
```

## Solutions (C)
```C
int* twoSum(int* nums, int numsSize, int target, int* returnSize){
    static int a[2]={0};
    
    for(int i=0;i<numsSize-1;i++)
    {
        for(int j=i+1;j<numsSize;j++)
        {
            if(nums[i]+nums[j] == target)
            {
                a[0]=i;
                a[1]=j;
                *returnSize=2;
                return a;       
            }
        }
    }
    *returnSize=0;
    return 0;
  
}
```

