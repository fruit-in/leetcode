# 189. 旋转数组
给定一个数组，将数组中的元素向右移动 *k* 个位置，其中 *k* 是非负数。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3,4,5,6,7] 和 k = 3
<strong>输出:</strong> [5,6,7,1,2,3,4]
<strong>解释:</strong>
向右旋转 1 步: [7,1,2,3,4,5,6]
向右旋转 2 步: [6,7,1,2,3,4,5]
向右旋转 3 步: [5,6,7,1,2,3,4]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [-1,-100,3,99] 和 k = 2
<strong>输出:</strong> [3,99,-1,-100]
<strong>解释:</strong>
向右旋转 1 步: [99,-1,-100,3]
向右旋转 2 步: [3,99,-1,-100]
</pre>

#### 说明:
* 尽可能想出更多的解决方案，至少有三种不同的方法可以解决这个问题。
* 要求使用空间复杂度为 O(1) 的 **原地** 算法。

## 题解 (Rust)

### 1. 暴力法
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

### 2. 保存后k个数字
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

### 3. 反转
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

### 4. 环状替换
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
