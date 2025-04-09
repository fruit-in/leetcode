# 2528. 最大化城市的最小电量
给你一个下标从 **0** 开始长度为 `n` 的整数数组 `stations` ，其中 `stations[i]` 表示第 `i` 座城市的供电站数目。

每个供电站可以在一定 **范围** 内给所有城市提供电力。换句话说，如果给定的范围是 `r` ，在城市 `i` 处的供电站可以给所有满足 `|i - j| <= r` 且 `0 <= i, j <= n - 1` 的城市 `j` 供电。

* `|x|` 表示 `x` 的 **绝对值** 。比方说，`|7 - 5| = 2` ，`|3 - 10| = 7` 。

一座城市的 **电量** 是所有能给它供电的供电站数目。

政府批准了可以额外建造 `k` 座供电站，你需要决定这些供电站分别应该建在哪里，这些供电站与已经存在的供电站有相同的供电范围。

给你两个整数 `r` 和 `k` ，如果以最优策略建造额外的发电站，返回所有城市中，最小电量的最大值是多少。

这 `k` 座供电站可以建在多个城市。

#### 示例 1:
<pre>
<strong>输入:</strong> stations = [1,2,4,5,0], r = 1, k = 2
<strong>输出:</strong> 5
<strong>解释:</strong>
最优方案之一是把 2 座供电站都建在城市 1 。
每座城市的供电站数目分别为 [1,4,4,5,0] 。
- 城市 0 的供电站数目为 1 + 4 = 5 。
- 城市 1 的供电站数目为 1 + 4 + 4 = 9 。
- 城市 2 的供电站数目为 4 + 4 + 5 = 13 。
- 城市 3 的供电站数目为 5 + 4 = 9 。
- 城市 4 的供电站数目为 5 + 0 = 5 。
供电站数目最少是 5 。
无法得到更优解，所以我们返回 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> stations = [4,4,4,4], r = 0, k = 3
<strong>输出:</strong> 4
<strong>解释:</strong>
无论如何安排，总有一座城市的供电站数目是 4 ，所以最优解是 4 。
</pre>

#### 提示:
* `n == stations.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= stations[i] <= 10<sup>5</sup></code>
* `0 <= r <= n - 1`
* <code>0 <= k <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let stations = stations.into_iter().map(|p| p as i64).collect::<Vec<_>>();
        let r = r as usize;
        let k = k as i64;
        let n = stations.len();
        let mut lo = i64::MAX;
        let mut hi = i64::MIN;

        for i in 0..n {
            lo = lo.min(stations[i]);
            hi = hi.max(stations[i] * (r as i64 * 2 + 1) + k);
        }

        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            let mut new_stations = stations.clone();
            let mut power = (0..r).map(|i| stations[i]).sum::<i64>();
            let mut build = 0;

            for i in 0..stations.len() {
                if i > r {
                    power -= new_stations[i - r - 1];
                }
                if i + r < n {
                    power += new_stations[i + r];
                }
                if power < mid {
                    new_stations[(i + r).min(n - 1)] += mid - power;
                    build += mid - power;
                    power = mid;
                }

                if build > k {
                    break;
                }
            }

            if build <= k {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        lo
    }
}
```
