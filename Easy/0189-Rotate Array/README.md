# 189. Rotate Array
Given an array, rotate the array to the right by *k* steps, where *k* is non-negative.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3,4,5,6,7] and k = 3
<strong>Output:</strong> [5,6,7,1,2,3,4]
<strong>Explanation:</strong>
rotate 1 steps to the right: [7,1,2,3,4,5,6]
rotate 2 steps to the right: [6,7,1,2,3,4,5]
rotate 3 steps to the right: [5,6,7,1,2,3,4]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [-1,-100,3,99] and k = 2
<strong>Output:</strong> [3,99,-1,-100]
<strong>Explanation:</strong>
rotate 1 steps to the right: [99,-1,-100,3]
rotate 2 steps to the right: [3,99,-1,-100]
</pre>

#### Note:
* Try to come up as many solutions as you can, there are at least 3 different ways to solve this problem.
* Could you do it in-place with O(1) extra space?

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            for i in (1..nums.len()).rev() {
                nums.swap(i, i - 1);
            }
        }
    }
}
```

### 2. Store Last k Numbers
```Rust
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let last_k = nums[(nums.len() - k)..].to_vec();
        for i in (k..nums.len()).rev() {
            nums.swap(i, i - k);
        }
        nums.splice(..k, last_k);
    }
}
```

### 3. Reverse
```Rust
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}
```

### 4. Cyclic Replacements
```Rust
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let mut start = 0;
        let mut cnt = 0;
        while cnt < nums.len() {
            let mut cur = start;
            let mut prev = nums[cur];
            loop {
                let next = (cur + k) % nums.len();
                let temp = nums[next];
                nums[next] = prev;
                cur = next;
                prev = temp;
                cnt += 1;
                
                if start == cur {
                    break;
                }
            }
            start += 1;
        }
    }
}
```
