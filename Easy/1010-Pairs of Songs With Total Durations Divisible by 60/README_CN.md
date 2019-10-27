# 1010. 总持续时间可被 60 整除的歌曲
在歌曲列表中，第 ```i``` 首歌曲的持续时间为 ```time[i]``` 秒。

返回其总持续时间（以秒为单位）可被 ```60``` 整除的歌曲对的数量。形式上，我们希望索引的数字  ```i < j``` 且有 ```(time[i] + time[j]) % 60 == 0```。

#### 示例 1:
<pre>
<strong>输入:</strong> [30,20,150,100,40]
<strong>输出:</strong> 3
<strong>解释:</strong> 这三对的总持续时间可被 60 整数：
(time[0] = 30, time[2] = 150): 总持续时间 180
(time[1] = 20, time[3] = 100): 总持续时间 120
(time[1] = 20, time[4] = 40): 总持续时间 60
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [60,60,60]
<strong>输出:</strong> 3
<strong>解释:</strong> 所有三对的总持续时间都是 120，可以被 60 整数。
</pre>

#### 提示:
1. ```1 <= time.length <= 60000```
2. ```1 <= time[i] <= 500```

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 0..time.len() {
            for j in (i + 1)..time.len() {
                if (time[i] + time[j]) % 60 == 0 {
                    ret += 1;
                }
            }
        }

        ret
    }
}
```

### 2. 计数
```Rust
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut cnt = [0; 60];
        let mut ret = 0;

        for t in time {
            ret += match t % 60 {
                0 => cnt[0],
                _ => cnt[60 - t as usize % 60],
            };
            cnt[t as usize % 60] += 1;
        }

        ret
    }
}
```
