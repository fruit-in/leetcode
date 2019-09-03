# 1. 两数之和
给定一个整数数组 ```nums``` 和一个目标值 ```target```，请你在该数组中找出和为目标值的那 **两个** 整数，并返回他们的数组下标。

你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。

#### 示例:
<pre>
给定 nums = [2, 7, 11, 15], target = 9

因为 nums[<strong>0</strong>] + nums[<strong>1</strong>] = 2 + 7 = 9
所以返回 [<strong>0, 1</strong>]
</pre>

## 题解 (Python)

### 1. 暴力法
```Python3
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for k, v in enumerate(nums):
            if target - v in nums[k + 1:]:
                return [k, k + 1 + nums[k + 1:].index(target - v)]
```

### 2. 单次遍历哈希表
```Python3
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        s = {}
        for k, v in enumerate(nums):
            if target - v in s.keys():
                return [k, s[target - v]]
            s[v] = k
```

### 3. 两次遍历哈希表
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

## 题解 (Rust)

### 1. 暴力法
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

### 2. 单次遍历哈希表
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

### 3. 两次遍历哈希表
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
