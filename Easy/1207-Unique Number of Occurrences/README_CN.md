# 1207. 独一无二的出现次数
给你一个整数数组 ```arr```，请你帮忙统计数组中每个数的出现次数。

如果每个数的出现次数都是独一无二的，就返回 ```true```；否则返回 ```false```。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,2,2,1,1,3]
<strong>输出:</strong> true
<strong>解释:</strong> 在该数组中，1 出现了 3 次，2 出现了 2 次，3 只出现了 1 次。没有两个数的出现次数相同。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,2]
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [-3,0,1,-3,1,1,1,-3,10,0]
<strong>输出:</strong> true
</pre>

#### 提示:
* ```1 <= arr.length <= 1000```
* ```-1000 <= arr[i] <= 1000```

## 题解 (Rust)

### 1. 哈希表和集合
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in arr {
            *map.entry(n).or_insert(0) += 1;
        }

        let set: HashSet<_> = map.values().collect();

        set.len() == map.len()
    }
}
```

### 2. 排序
```Rust
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();

        let mut times = [false; 1001];
        let mut i = 0;
        let mut j = 1;
        while i < arr.len() {
            while j < arr.len() && arr[i] == arr[j] {
                j += 1;
            }
            if times[j - i] {
                return false;
            }
            times[j - i] = true;
            i = j;
        }

        true
    }
}
```
