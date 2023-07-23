# 697. 数组的度
给定一个非空且只包含非负数的整数数组 ```nums```, 数组的度的定义是指数组里任一元素出现频数的最大值。

你的任务是找到与 ```nums``` 拥有相同大小的度的最短连续子数组，返回其长度。

#### 示例 1:
<pre>
<strong>输入:</strong> [1, 2, 2, 3, 1]
<strong>输出:</strong> 2
<strong>解释:</strong>
输入数组的度是2，因为元素1和2的出现频数最大，均为2.
连续子数组里面拥有相同度的有如下所示:
[1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
最短连续子数组[2, 2]的长度为2，所以返回2.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,2,2,3,1,4,2]
<strong>输出:</strong> 6
</pre>

#### 注意:
* ```nums.length``` 在1到50,000区间范围内。
* ```nums[i]``` 是一个在0到49,999范围内的整数。

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut cnt_left_len = HashMap::new();
        let mut degree = 0;

        for i in 0..nums.len() {
            let cll = cnt_left_len.entry(nums[i]).or_insert([0, i, 1]);
            cll[0] += 1;
            cll[2] = i - cll[1] + 1;
            degree = degree.max(cll[0])
        }

        cnt_left_len.values()
                    .filter(|arr| arr[0] == degree)
                    .map(|arr| arr[2])
                    .min()
                    .unwrap() as i32
    }
}
```
