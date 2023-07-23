# 264. 丑数 II
编写一个程序，找出第 ```n``` 个丑数。

丑数就是只包含质因数 ```2, 3, 5``` 的**正整数**。

#### 示例:
<pre>
<strong>输入:</strong> n = 10
<strong>输出:</strong> 12
<strong>解释:</strong> 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 是前 10 个丑数。
</pre>

#### 说明:
1. ```1``` 是丑数。
2. ```n``` **不超过**1690。

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut uglys = vec![1];
        let mut p2 = 0;
        let mut p3 = 0;
        let mut p5 = 0;

        for _ in 1..n {
            let min_ugly = (2 * uglys[p2]).min(3 * uglys[p3]).min(5 * uglys[p5]);
            uglys.push(min_ugly);

            if min_ugly == 2 * uglys[p2] {
                p2 += 1;
            }
            if min_ugly == 3 * uglys[p3] {
                p3 += 1;
            }
            if min_ugly == 5 * uglys[p5] {
                p5 += 1;
            }
        }

        uglys[n as usize - 1]
    }
}
```
