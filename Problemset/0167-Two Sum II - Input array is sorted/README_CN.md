# 167. 两数之和 II - 输入有序数组
给定一个已按照***升序排列*** 的有序数组，找到两个数使得它们相加之和等于目标数。

函数应该返回这两个下标值 index1 和 index2，其中 index1 必须小于 index2。

#### 说明:
* 返回的下标值（index1 和 index2）不是从零开始的。
* 你可以假设每个输入只对应唯一的答案，而且你不可以重复使用相同的元素。

#### 示例:
<pre>
<strong>输入:</strong> numbers = [2,7,11,15], target = 9
<strong>输出:</strong> [1, 2]
<strong>解释:</strong> 2 与 7 之和等于目标数 9 。因此 index1 = 1, index2 = 2 。
</pre>

## 题解 (Rust)

### 1. 二分查找
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

### 2. 双指针
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
