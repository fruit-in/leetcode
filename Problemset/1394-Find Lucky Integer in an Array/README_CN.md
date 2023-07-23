# 1394. 找出数组中的幸运数
在整数数组中，如果一个整数的出现频次和它的数值大小相等，我们就称这个整数为「幸运数」。

给你一个整数数组 ```arr```，请你从中找出并返回一个幸运数。
* 如果数组中存在多个幸运数，只需返回 **最大** 的那个。
* 如果数组中不含幸运数，则返回 **-1** 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [2,2,3,4]
<strong>输出:</strong> 2
<strong>解释:</strong> 数组中唯一的幸运数是 2 ，因为数值 2 的出现频次也是 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,2,2,3,3,3]
<strong>输出:</strong> 3
<strong>解释:</strong> 1、2 以及 3 都是幸运数，只需要返回其中最大的 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [2,2,2,3,3]
<strong>输出:</strong> -1
<strong>解释:</strong> 数组中不存在幸运数。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [5]
<strong>输出:</strong> -1
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> arr = [7,7,7,7,7,7,7]
<strong>输出:</strong> 7
</pre>

#### 提示:
* ```1 <= arr.length <= 500```
* ```1 <= arr[i] <= 500```

## 题解 (Rust)

### 1. 计数
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();

        for n in arr {
            *cnt.entry(n).or_insert(0) += 1;
        }

        match cnt.iter().filter(|(k, v)| k == v).max_by_key(|(&k, &v)| k) {
            Some((&k, &v)) => k,
            None => -1,
        }
    }
}
```
