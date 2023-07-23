# 1011. 在 D 天内送达包裹的能力
传送带上的包裹必须在 D 天内从一个港口运送到另一个港口。

传送带上的第 `i` 个包裹的重量为 `weights[i]`。每一天，我们都会按给出重量的顺序往传送带上装载包裹。我们装载的重量不会超过船的最大运载重量。

返回能在 `D` 天内将传送带上的所有包裹送达的船的最低运载能力。

#### 示例 1:
<pre>
<strong>输入:</strong> weights = [1,2,3,4,5,6,7,8,9,10], D = 5
<strong>输出:</strong> 15
<strong>解释:</strong>
船舶最低载重 15 就能够在 5 天内送达所有包裹，如下所示：
第 1 天：1, 2, 3, 4, 5
第 2 天：6, 7
第 3 天：8
第 4 天：9
第 5 天：10

请注意，货物必须按照给定的顺序装运，因此使用载重能力为 14 的船舶并将包装分成 (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) 是不允许的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> weights = [3,2,2,4,1,4], D = 3
<strong>输出:</strong> 6
<strong>解释:</strong>
船舶最低载重 6 就能够在 3 天内送达所有包裹，如下所示：
第 1 天：3, 2
第 2 天：2, 4
第 3 天：1, 4
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> weights = [1,2,3,1,1], D = 4
<strong>输出:</strong> 3
<strong>解释:</strong>
第 1 天：1
第 2 天：2
第 3 天：3
第 4 天：1, 1
</pre>

#### 提示:
* <code>1 <= D <= weights.length <= 5 * 10<sup>4</sup></code>
* `1 <= weights[i] <= 500`

## 题解 (Rust)

### 1. 二分查找
```Rust
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let max_weight = *weights.iter().max().unwrap();
        let sum_weight = weights.iter().sum::<i32>();
        let capacities = (max_weight..=sum_weight).collect::<Vec<_>>();

        match capacities.binary_search_by_key(&false, |&c| Self::shipped_in_time(&weights, c, days))
        {
            Ok(c) => c as i32 + 1 + max_weight,
            Err(c) => c as i32 + max_weight,
        }
    }

    fn shipped_in_time(weights: &[i32], capacity: i32, days: i32) -> bool {
        let mut remain = 0;
        let mut spend = 0;

        for &w in weights {
            if remain < w {
                remain = capacity;
                spend += 1;
            }
            remain -= w;
        }

        spend <= days
    }
}
```
