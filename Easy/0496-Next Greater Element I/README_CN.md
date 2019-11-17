# 496. 下一个更大元素 I
给定两个**没有重复元素**的数组 ```nums1``` 和 ```nums2``` ，其中```nums1``` 是 ```nums2``` 的子集。找到 ```nums1``` 中每个元素在 ```nums2``` 中的下一个比其大的值。

```nums1``` 中数字 **x** 的下一个更大元素是指 **x** 在 ```nums2``` 中对应位置的右边的第一个比 **x** 大的元素。如果不存在，对应位置输出-1。

#### 示例 1:
<pre>
<strong>输入:</strong> <strong>nums1</strong> = [4,1,2], <strong>nums2</strong> = [1,3,4,2].
<strong>输出:</strong> [-1,3,-1]
<strong>解释:</strong>
    对于num1中的数字4，你无法在第二个数组中找到下一个更大的数字，因此输出 -1。
    对于num1中的数字1，第二个数组中数字1右边的下一个较大数字是 3。
    对于num1中的数字2，第二个数组中没有下一个更大的数字，因此输出 -1。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> <strong>nums1</strong> = [2,4], <strong>nums2</strong> = [1,2,3,4].
<strong>输出:</strong> [3,-1]
<strong>解释:</strong>
    对于num1中的数字2，第二个数组中的下一个较大数字是3。
    对于num1中的数字4，第二个数组中没有下一个更大的数字，因此输出 -1。
</pre>

#### 注意:
1. ```nums1```和```nums2```中所有元素是唯一的。
2. ```nums1```和```nums2``` 的数组大小都不超过1000。

## 题解 (Rust)

### 1. 暴力
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

### 2. 栈和哈希表
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
