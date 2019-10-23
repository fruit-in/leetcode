# 344. 反转字符串
编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 ```char[]``` 的形式给出。

不要给另外的数组分配额外的空间，你必须**[原地](https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95)修改输入数组**、使用 O(1) 的额外空间解决这一问题。

你可以假设数组中的所有字符都是 [ASCII](https://baike.baidu.com/item/ASCII) 码表中的可打印字符。

#### 示例 1:
<pre>
<strong>输入:</strong> ["h","e","l","l","o"]
<strong>输出:</strong> ["o","l","l","e","h"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ["H","a","n","n","a","h"]
<strong>输出:</strong> ["h","a","n","n","a","H"]
</pre>

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() > 0 {
            let mut left = 0;
            let mut right = s.len() - 1;
            while left < right {
                s.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
    }
}
```
