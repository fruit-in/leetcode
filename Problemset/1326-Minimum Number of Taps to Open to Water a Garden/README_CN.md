# 1326. 灌溉花园的最少水龙头数目
在 x 轴上有一个一维的花园。花园长度为 `n`，从点 `0` 开始，到点 `n` 结束。

花园里总共有 `n + 1` 个水龙头，分别位于 `[0, 1, ..., n]` 。

给你一个整数 `n` 和一个长度为 `n + 1` 的整数数组 `ranges` ，其中 `ranges[i]` （下标从 0 开始）表示：如果打开点 `i` 处的水龙头，可以灌溉的区域为 `[i -  ranges[i], i + ranges[i]]` 。

请你返回可以灌溉整个花园的 **最少水龙头数目** 。如果花园始终存在无法灌溉到的地方，请你返回 **-1** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/01/16/1685_example_1.png)
<pre>
<strong>输入:</strong> n = 5, ranges = [3,4,1,1,0,0]
<strong>输出:</strong> 1
<strong>解释:</strong>
点 0 处的水龙头可以灌溉区间 [-3,3]
点 1 处的水龙头可以灌溉区间 [-3,5]
点 2 处的水龙头可以灌溉区间 [1,3]
点 3 处的水龙头可以灌溉区间 [2,4]
点 4 处的水龙头可以灌溉区间 [4,4]
点 5 处的水龙头可以灌溉区间 [5,5]
只需要打开点 1 处的水龙头即可灌溉整个花园 [0,5] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, ranges = [0,0,0,0]
<strong>输出:</strong> -1
<strong>解释:</strong> 即使打开所有水龙头，你也无法灌溉整个花园。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>4</sup></code>
* `ranges.length == n + 1`
* `0 <= ranges[i] <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut new_ranges = vec![];
        let mut i = 0;
        let mut ret = 0;

        for (j, &r) in ranges.iter().enumerate() {
            let (start1, end1) = (j as i32 - r, j as i32 + r);
            let mut flag = true;

            while let Some(&(start2, end2)) = new_ranges.last() {
                if start1 <= start2 {
                    new_ranges.pop();
                    continue;
                }
                flag = end2 < end1;
                break;
            }

            if flag {
                new_ranges.push((start1, end1));
            }
        }

        for j in 0..new_ranges.len() {
            if i >= n {
                break;
            } else if new_ranges[j].0 > i {
                return -1;
            } else if j == new_ranges.len() - 1 || new_ranges[j + 1].0 > i {
                i = new_ranges[j].1;
                ret += 1;
            }
        }

        if i < n {
            return -1;
        }

        ret
    }
}
```
