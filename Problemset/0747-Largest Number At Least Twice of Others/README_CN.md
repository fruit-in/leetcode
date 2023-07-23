# 747. 至少是其他数字两倍的最大数
在一个给定的数组```nums```中，总是存在一个最大元素 。

查找数组中的最大元素是否至少是数组中每个其他数字的两倍。

如果是，则返回最大元素的索引，否则返回-1。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3, 6, 1, 0]
<strong>输出:</strong> 1
<strong>解释:</strong> 6是最大的整数, 对于数组中的其他整数,
6大于数组中其他元素的两倍。6的索引是1, 所以我们返回1.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1, 2, 3, 4]
<strong>输出:</strong> -1
<strong>解释:</strong> 4没有超过3的两倍大, 所以我们返回 -1.
</pre>

#### 提示:
1. ```nums``` 的长度范围在```[1, 50]```.
2. 每个 ```nums[i]``` 的整数范围在 ```[0, 100]```.

## 题解 (Rust)

### 1. 线性扫描
```Rust
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let m = *nums.iter().max().unwrap();
        if nums.iter().all(|&x| 2 * x <= m || x == m) {
            nums.iter().position(|&x| x == m).unwrap() as i32
        } else {
            -1
        }
    }
}
```
