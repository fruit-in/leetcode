# 1387. 将整数按权重排序
我们将整数 ```x``` 的 **权重** 定义为按照下述规则将 ```x``` 变成 ```1``` 所需要的步数：
* 如果 ```x``` 是偶数，那么 ```x = x / 2```
* 如果 ```x``` 是奇数，那么 ```x = 3 * x + 1```

比方说，x=3 的权重为 7 。因为 3 需要 7 步变成 1 （3 --> 10 --> 5 --> 16 --> 8 --> 4 --> 2 --> 1）。

给你三个整数 ```lo```， ```hi``` 和 ```k``` 。你的任务是将区间 ```[lo, hi]``` 之间的整数按照它们的权重 **升序排序** ，如果大于等于 2 个整数有 **相同** 的权重，那么按照数字自身的数值 **升序排序** 。

请你返回区间 ```[lo, hi]``` 之间的整数按权重排序后的第 ```k``` 个数。

注意，题目保证对于任意整数 ```x``` ```（lo <= x <= hi）``` ，它变成 ```1``` 所需要的步数是一个 32 位有符号整数。

#### 示例 1:
<pre>
<strong>输入:</strong> lo = 12, hi = 15, k = 2
<strong>输出:</strong> 13
<strong>解释:</strong> 12 的权重为 9（12 --> 6 --> 3 --> 10 --> 5 --> 16 --> 8 --> 4 --> 2 --> 1）
13 的权重为 9
14 的权重为 17
15 的权重为 17
区间内的数按权重排序以后的结果为 [12,13,14,15] 。对于 k = 2 ，答案是第二个整数也就是 13 。
注意，12 和 13 有相同的权重，所以我们按照它们本身升序排序。14 和 15 同理。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> lo = 1, hi = 1, k = 1
<strong>输出:</strong> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> lo = 7, hi = 11, k = 4
<strong>输出:</strong> 7
<strong>解释:</strong> 区间内整数 [7, 8, 9, 10, 11] 对应的权重为 [16, 3, 19, 6, 14] 。
按权重排序后得到的结果为 [8, 10, 11, 7, 9] 。
排序后数组中第 4 个数字为 7 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> lo = 10, hi = 20, k = 5
<strong>输出:</strong> 13
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> lo = 1, hi = 1000, k = 777
<strong>输出:</strong> 570
</pre>

#### 提示:
* ```1 <= lo <= hi <= 1000```
* ```1 <= k <= hi - lo + 1```

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut arr = (lo..=hi).collect::<Vec<i32>>();
        let mut stack = arr.clone();
        let mut power = HashMap::new();
        power.insert(1, 0);

        while let Some(x) = stack.pop() {
            if !power.contains_key(&x) {
                let y = match x % 2 {
                    0 => x / 2,
                    _ => 3 * x + 1,
                };
                match power.get(&y) {
                    Some(z) => { power.insert(x, z + 1); },
                    None => {
                        stack.push(x);
                        stack.push(y);
                    },
                }
            }
        }

        arr.sort_by_key(|x| *power.get(&x).unwrap());

        arr[k as usize - 1]
    }
}
```
