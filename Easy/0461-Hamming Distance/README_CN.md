# 461. 汉明距离
两个整数之间的[汉明距离](https://baike.baidu.com/item/%E6%B1%89%E6%98%8E%E8%B7%9D%E7%A6%BB)指的是这两个数字对应二进制位不同的位置的数目。

给出两个整数 ```x``` 和 ```y```，计算它们之间的汉明距离。

#### 注意:
0 ≤ ```x```, ```y``` < 2<sup>31</sup>.

#### 示例:
<pre>
<strong>输入:</strong> x = 1, y = 4
<strong>输出:</strong> 2
<strong>解释:</strong>
1   (0 0 0 1)
4   (0 1 0 0)
       ↑   ↑

上面的箭头指出了对应二进制位不同的位置。
</pre>

## 题解 (Rust)

### 1. 异或
```Rust
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut counter = 0;
        let mut z = x ^ y;
        while z != 0 {
            counter += z & 1;
            z >>= 1;
        }
        counter
    }
}
```
