# 2279. 装满石头的背包的最大数量
现有编号从 `0` 到 `n - 1` 的 `n` 个背包。给你两个下标从 **0** 开始的整数数组 `capacity` 和 `rocks` 。第 `i` 个背包最大可以装 `capacity[i]` 块石头，当前已经装了 `rocks[i]` 块石头。另给你一个整数 `additionalRocks` ，表示你可以放置的额外石头数量，石头可以往 **任意** 背包中放置。

请你将额外的石头放入一些背包中，并返回放置后装满石头的背包的 **最大** 数量。

#### 示例 1:
<pre>
<strong>输入:</strong> capacity = [2,3,4,5], rocks = [1,2,4,4], additionalRocks = 2
<strong>输出:</strong> 3
<strong>解释:</strong>
1 块石头放入背包 0 ，1 块石头放入背包 1 。
每个背包中的石头总数是 [2,3,4,4] 。
背包 0 、背包 1 和 背包 2 都装满石头。
总计 3 个背包装满石头，所以返回 3 。
可以证明不存在超过 3 个背包装满石头的情况。
注意，可能存在其他放置石头的方案同样能够得到 3 这个结果。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> capacity = [10,2,2], rocks = [2,2,0], additionalRocks = 100
<strong>输出:</strong> 3
<strong>解释:</strong>
8 块石头放入背包 0 ，2 块石头放入背包 2 。
每个背包中的石头总数是 [10,2,2] 。
背包 0 、背包 1 和背包 2 都装满石头。
总计 3 个背包装满石头，所以返回 3 。
可以证明不存在超过 3 个背包装满石头的情况。
注意，不必用完所有的额外石头。
</pre>

#### 提示:
* `n == capacity.length == rocks.length`
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= capacity[i] <= 10<sup>9</sup></code>
* `0 <= rocks[i] <= capacity[i]`
* <code>1 <= additionalRocks <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut remains = (0..rocks.len())
            .map(|i| capacity[i] - rocks[i])
            .collect::<Vec<_>>();
        let mut additional_rocks = additional_rocks;
        let mut ret = 0;
        remains.sort_unstable();

        for remain in remains {
            if remain > additional_rocks {
                break;
            }

            additional_rocks -= remain;
            ret += 1;
        }

        ret
    }
}
```
