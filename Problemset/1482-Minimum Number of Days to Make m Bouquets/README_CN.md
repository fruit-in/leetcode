# 1482. 制作 m 束花所需的最少天数
给你一个整数数组 `bloomDay`，以及两个整数 `m` 和 `k` 。

现需要制作 `m` 束花。制作花束时，需要使用花园中 **相邻的** `k` **朵花** 。

花园中有 `n` 朵花，第 `i` 朵花会在 `bloomDay[i]` 时盛开，**恰好** 可以用于 **一束** 花中。

请你返回从花园中摘 `m` 束花需要等待的最少的天数。如果不能摘到 `m` 束花则返回 **-1** 。

#### 示例 1:
<pre>
<strong>输入:</strong> bloomDay = [1,10,3,10,2], m = 3, k = 1
<strong>输出:</strong> 3
<strong>解释:</strong> 让我们一起观察这三天的花开过程，x 表示花开，而 _ 表示花还未开。
现在需要制作 3 束花，每束只需要 1 朵。
1 天后：[x, _, _, _, _]   // 只能制作 1 束花
2 天后：[x, _, _, _, x]   // 只能制作 2 束花
3 天后：[x, _, x, _, x]   // 可以制作 3 束花，答案为 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> bloomDay = [1,10,3,10,2], m = 3, k = 2
<strong>输出:</strong> -1
<strong>解释:</strong> 要制作 3 束花，每束需要 2 朵花，也就是一共需要 6 朵花。而花园中只有 5 朵花，无法满足制作要求，返回 -1 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> bloomDay = [7,7,7,7,12,7,7], m = 2, k = 3
<strong>输出:</strong> 12
<strong>解释:</strong> 要制作 2 束花，每束需要 3 朵。
花园在 7 天后和 12 天后的情况如下：
7 天后：[x, x, x, x, _, x, x]
可以用前 3 朵盛开的花制作第一束花。但不能使用后 3 朵盛开的花，因为它们不相邻。
12 天后：[x, x, x, x, x, x, x]
显然，我们可以用不同的方式制作两束花。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> bloomDay = [1000000000,1000000000], m = 1, k = 1
<strong>输出:</strong> 1000000000
<strong>解释:</strong> 需要等 1000000000 天才能采到花来制作花束
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> bloomDay = [1,10,2,9,3,8,4,7,5,6], m = 4, k = 2
<strong>输出:</strong> 9
</pre>

#### 提示:
* `bloomDay.length == n`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= bloomDay[i] <= 10<sup>9</sup></code>
* <code>1 <= m <= 10<sup>6</sup></code>
* `1 <= k <= n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if m as i64 * k as i64 > bloom_day.len() as i64 {
            return -1;
        }

        let mut low = *bloom_day.iter().min().unwrap();
        let mut high = *bloom_day.iter().max().unwrap();

        while low < high {
            let day = (low + high) / 2;
            let mut bouquets = 0;
            let mut i = 0;
            let mut j = 0;

            while j < bloom_day.len() {
                if bloom_day[i] > day && bloom_day[j] <= day {
                    i = j;
                }

                if bloom_day[j] <= day && j - i + 1 == k as usize {
                    bouquets += 1;
                    i = j + 1;
                } else if bloom_day[j] > day {
                    i = j + 1;
                }

                j += 1;
            }

            if bouquets < m {
                low = day + 1;
            } else {
                high = day;
            }
        }

        high
    }
}
```
