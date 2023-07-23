# 704. 二分查找
给定一个 ```n``` 个元素有序的（升序）整型数组 ```nums``` 和一个目标值 ```target```  ，写一个函数搜索 ```nums``` 中的 ```target```，如果目标值存在返回下标，否则返回 ```-1```。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [-1,0,3,5,9,12], target = 9
<strong>输出:</strong> 4
<strong>解释:</strong> 9 出现在 nums 中并且下标为 4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-1,0,3,5,9,12], target = 2
<strong>输出:</strong> -1
<strong>解释:</strong> 2 不存在 nums 中因此返回 -1
</pre>

#### 提示:
1. 你可以假设 ```nums``` 中的所有元素是不重复的。
2. ```n``` 将在 ```[1, 10000]```之间。
3. ```nums``` 的每个元素都将在 ```[-9999, 9999]```之间。

## 题解 (Rust)

### 1. 二分查找
```Rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut head = 0;
        let mut tail = nums.len() as i32 - 1;
        while head <= tail {
            if target == nums[(head + tail) as usize / 2] {
                return (head + tail) / 2;
            } else if target > nums[(head + tail) as usize / 2] {
                head = (head + tail) / 2 + 1;
            } else if target < nums[(head + tail) as usize / 2] {
                tail = (head + tail) / 2 - 1;
            }
        }
        -1
    }
}
```
