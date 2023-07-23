# 970. 强整数
给定两个正整数 ```x``` 和 ```y```，如果某一整数等于 ```x^i + y^j```，其中整数 ```i >= 0``` 且 ```j >= 0```，那么我们认为该整数是一个*强整数*。

返回值小于或等于 ```bound``` 的所有*强整数*组成的列表。

你可以按任何顺序返回答案。在你的回答中，每个值最多出现一次。

#### 示例 1:
<pre>
<strong>输入:</strong> x = 2, y = 3, bound = 10
<strong>输出:</strong> [2,3,4,5,7,9,10]
<strong>解释:</strong>
2 = 2^0 + 3^0
3 = 2^1 + 3^0
4 = 2^0 + 3^1
5 = 2^1 + 3^1
7 = 2^2 + 3^1
9 = 2^3 + 3^0
10 = 2^0 + 3^2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> x = 3, y = 5, bound = 15
<strong>输出:</strong> [2,4,6,8,10,14]
</pre>

#### 提示:
* ```1 <= x <= 100```
* ```1 <= y <= 100```
* ```0 <= bound <= 10^6```

## 题解 (Rust)

### 1. 暴力法
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut set = HashSet::new();

        for i in 0..20 {
            for j in 0..20 {
                let pow_int = x.saturating_pow(i).saturating_add(y.saturating_pow(j));

                if pow_int <= bound {
                    set.insert(pow_int);
                }
            }
        }

        set.drain().collect()
    }
}
```
