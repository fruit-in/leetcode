# 371. 两整数之和
**不使用**运算符 ```+``` 和 ```-``` ，计算两整数 ```a``` 、```b``` 之和。

#### 示例 1:
<pre>
<strong>输入:</strong> a = 1, b = 2
<strong>输出:</strong> 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> a = -2, b = 3
<strong>输出:</strong> 1
</pre>

## 题解 (Rust)

### 1. 位操作
```Rust
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut ans = 0;
        let mut mask = 1;
        let mut c = false;
        for _ in 0..32 {
            if (a & mask == b & mask && c) || (a & mask != b & mask && !c) {
                ans |= mask;
            }
            if a & mask != 0 && b & mask != 0 {
                c = true;
            }
            if a & mask == 0 && b & mask == 0 {
                c = false;
            }
            mask <<= 1;
        }
        ans
    }
}
```
