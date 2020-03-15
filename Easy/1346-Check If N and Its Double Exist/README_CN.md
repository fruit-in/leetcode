# 1346. 检查整数及其两倍数是否存在
给你一个整数数组 ```arr```，请你检查是否存在两个整数 ```N``` 和 ```M```，满足 ```N``` 是 ```M``` 的两倍（即，```N = 2 * M```）。

更正式地，检查是否存在两个下标 ```i``` 和 ```j``` 满足：
* ```i != j```
* ```0 <= i, j < arr.length```
* ```arr[i] == 2 * arr[j]```

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [10,2,5,3]
<strong>输出:</strong> true
<strong>解释:</strong> N = 10 是 M = 5 的两倍，即 10 = 2 * 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [7,1,14,11]
<strong>输出:</strong> true
<strong>解释:</strong> N = 14 是 M = 7 的两倍，即 14 = 2 * 7 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [3,1,7,11]
<strong>输出:</strong> false
<strong>解释:</strong> 在该情况下不存在 N 和 M 满足 N = 2 * M 。
</pre>

#### 提示:
* ```2 <= arr.length <= 500```
* ```-10^3 <= arr[i] <= 10^3```

## 题解 (Rust)

### 1. 集合
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for n in arr {
            if set.contains(&(2 * n)) || (n % 2 == 0 && set.contains(&(n / 2))) {
                return true;
            }
            set.insert(n);
        }

        false
    }
}
```
