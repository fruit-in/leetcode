# 912. 排序数组
给定一个整数数组 ```nums```，将该数组升序排列。

#### 示例 1:
<pre>
<strong>输入:</strong> [5,2,3,1]
<strong>输出:</strong> [1,2,3,5]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [5,1,1,2,0,0]
<strong>输出:</strong> [0,0,1,1,2,5]
</pre>

#### 提示:
1. <code>1 <= A.length <= 10000</code>
2. <code>-50000 <= A[i] <= 50000</code>

## 题解 (Rust)

### 1. 冒泡排序
```Rust
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut nums = nums;
        for i in 0..(len - 1) {
            for j in 0..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                }
            }
        }
        nums
    }
}
```

### 2. 选择排序
```Rust
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut nums = nums;
        for i in 0..(len - 1) {
            let mut min_index = i;
            for j in (i + 1)..len {
                if nums[j] < nums[min_index] {
                    min_index = j;
                }
            }
            nums.swap(i, min_index);
        }
        nums
    }
}
```

### 3. 插入排序
```Rust
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut nums = nums;
        for i in 1..len {
            let val = nums[i];
            let mut j = i;
            while j > 0 && nums[j - 1] > val {
                nums[j] = nums[j - 1];
                j -= 1;
            }
            nums[j] = val;
        }
        nums
    }
}
```

### 4. 归并排序
```Rust
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len < 2 {
            nums
        } else {
            Self::merge(Self::sort_array(nums[0..(len / 2)].to_vec()),
                Self::sort_array(nums[(len / 2)..].to_vec()))
        }
    }

    pub fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
        let mut new_array = Vec::new();
        while left.len() > 0 && right.len() > 0 {
            if left[0] < right[0] {
                new_array.push(left.remove(0));
            } else {
                new_array.push(right.remove(0));
            }
        }
        while left.len() > 0 {
            new_array.push(left.remove(0));
        }
        while right.len() > 0 {
            new_array.push(right.remove(0));
        }
        new_array
    }
}
```

### 5. 快速排序
```Rust
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return nums;
        }
        let mut nums = nums;
        let mut i = 1;
        for j in 1..nums.len() {
            if nums[j] < nums[0] {
                nums.swap(i, j);
                i += 1;
            }
        }
        nums.swap(i - 1, 0);
        let mut ret = Self::sort_array(nums[..i].to_vec());
        ret.append(&mut Self::sort_array(nums[i..].to_vec()));
        ret
    }
}
```

### 6. 计数排序
```Rust
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        let mut count = vec![0; (max - min) as usize + 1];
        let mut new_array = Vec::new();
        for n in nums {
            count[(n - min) as usize] += 1;
        }
        for i in 0..=(max - min) {
            for j in 0..count[i as usize] {
                new_array.push(min + i);
            }
        }
        new_array
    }
}
```

### 7. 基数排序
```Rust
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            nums[i] += 50000;
        }
        let mut bucket = vec![Vec::new(); 10];
        for i in 0..5 {
            for n in nums {
                let m = n % 10_i32.pow(i + 1) / 10_i32.pow(i);
                bucket[m as usize].push(n);
            }
            nums = Vec::new();
            for j in 0..10 {
                while bucket[j].len() > 0 {
                    nums.push(bucket[j].remove(0));
                }
            }
        }
        for i in 0..nums.len() {
            nums[i] -= 50000;
        }
        nums
    }
}
```
