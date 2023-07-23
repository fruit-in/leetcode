# 914. 卡牌分组
给定一副牌，每张牌上都写着一个整数。

此时，你需要选定一个数字 ```X```，使我们可以将整副牌按下述规则分成 1 组或更多组：
* 每组都有 ```X``` 张牌。
* 组内所有的牌上都写着相同的整数。

仅当你可选的 ```X >= 2``` 时返回 ```true```。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3,4,4,3,2,1]
<strong>输出:</strong> true
<strong>解释:</strong> 可行的分组是 [1,1]，[2,2]，[3,3]，[4,4]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,1,1,2,2,2,3,3]
<strong>输出:</strong> false
<strong>解释:</strong> 没有满足要求的分组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [1]
<strong>输出:</strong> false
<strong>解释:</strong> 没有满足要求的分组。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> [1,1]
<strong>输出:</strong> true
<strong>解释:</strong> 可行的分组是 [1,1]
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> [1,1,2,2,2,2]
<strong>输出:</strong> true
<strong>解释:</strong> 可行的分组是 [1,1]，[2,2]，[2,2]
</pre>

#### 提示:
1. ```1 <= deck.length <= 10000```
2. ```0 <= deck[i] < 10000```

## 题解 (Rust)

### 1. 暴力法
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in &deck {
            *map.entry(n).or_insert(0) += 1;
        }

        for x in (2..=(deck.len() / map.len())).filter(|x| deck.len() % x == 0) {
            if map.values().all(|v| v % x == 0) {
                return true;
            }
        }

        false
    }
}
```

### 2. GCD
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in deck {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut x = *map.values().nth(0).unwrap();
        map.values().fold(x, |x, y| Self::gcd(x, *y)) > 1
    }

    pub fn gcd(mut x: i32, mut y: i32) -> i32 {
        while x % y != 0 {
            let tmp = x;
            x = y;
            y = tmp % y;
        }
        y
    }
}
```
