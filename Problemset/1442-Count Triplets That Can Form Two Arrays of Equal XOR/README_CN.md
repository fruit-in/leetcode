# 1442. 形成两个异或相等数组的三元组数目
给你一个整数数组 `arr` 。

现需要从数组中取三个下标 `i`、`j` 和 `k` ，其中 `(0 <= i < j <= k < arr.length)` 。

`a` 和 `b` 定义如下：
* `a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]`
* `b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]`

注意：**^** 表示 **按位异或** 操作。

请返回能够令 `a == b` 成立的三元组 (`i`, `j` , `k`) 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [2,3,1,6,7]
<strong>输出:</strong> 4
<strong>解释:</strong> 满足题意的三元组分别是 (0,1,2), (0,2,2), (2,3,4) 以及 (2,4,4)
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,1,1,1,1]
<strong>输出:</strong> 10
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [2,3]
<strong>输出:</strong> 0
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [1,3,5,7,9]
<strong>输出:</strong> 3
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> arr = [7,11,12,9,5,2,7,17,22]
<strong>输出:</strong> 8
</pre>

#### 提示:
* `1 <= arr.length <= 300`
* <code>1 <= arr[i] <= 10<sup>8</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut prefix_xor = vec![0; arr.len() + 1];
        let mut prefix_xor_indices = HashMap::from([(0, vec![0])]);
        let mut ret = 0;

        for k in 0..arr.len() {
            prefix_xor[k + 1] = prefix_xor[k] ^ arr[k];
            for &i in prefix_xor_indices
                .get(&prefix_xor[k + 1])
                .unwrap_or(&vec![])
            {
                ret += k - i;
            }
            prefix_xor_indices
                .entry(prefix_xor[k + 1])
                .or_insert(vec![])
                .push(k + 1);
        }

        ret as i32
    }
}
```
