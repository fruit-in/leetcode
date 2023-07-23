# 476. 数字的补数
给定一个正整数，输出它的补数。补数是对该数的二进制表示取反。

#### 注意:
1. 给定的整数保证在32位带符号整数的范围内。
2. 你可以假定二进制数不包含前导零位。

#### 示例 1:
<pre>
<strong>输入:</strong> 5
<strong>输出:</strong> 2
<strong>解释:</strong> 5的二进制表示为101（没有前导零位），其补数为010。所以你需要输出2。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 1
<strong>输出:</strong> 0
<strong>解释:</strong> 1的二进制表示为1（没有前导零位），其补数为0。所以你需要输出0。
</pre>

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        2_i32.pow((num as f64).log2() as u32 + 1) - 1 - num
    }
}
```

### 2. 位操作
```Rust
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut ret = 0;

        for i in 0..30 {
            if num >> i == 0 {
                break;
            }
            if num & (1 << i) == 0 {
                ret ^= 1 << i;
            }
        }

        ret
    }
}
```
