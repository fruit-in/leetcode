# 2580. 统计将重叠区间合并成组的方案数
给你一个二维整数数组 `ranges` ，其中 <code>ranges[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> 表示 <code>start<sub>i</sub></code> 到 <code>end<sub>i</sub></code> 之间（包括二者）的所有整数都包含在第 `i` 个区间中。

你需要将 `ranges` 分成 **两个** 组（可以为空），满足：
* 每个区间只属于一个组。
* 两个有 **交集** 的区间必须在 **同一个** 组内。

如果两个区间有至少 **一个** 公共整数，那么这两个区间是 **有交集** 的。

* 比方说，区间 `[1, 3]` 和 `[2, 5]` 有交集，因为 `2` 和 `3` 在两个区间中都被包含。

请你返回将 `ranges` 划分成两个组的 **总方案数** 。由于答案可能很大，将它对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> ranges = [[6,10],[5,15]]
<strong>输出:</strong> 2
<strong>解释:</strong>
两个区间有交集，所以它们必须在同一个组内。
所以有两种方案：
- 将两个区间都放在第 1 个组中。
- 将两个区间都放在第 2 个组中。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ranges = [[1,3],[10,20],[2,5],[4,8]]
<strong>输出:</strong> 4
<strong>解释:</strong>
区间 [1,3] 和 [2,5] 有交集，所以它们必须在同一个组中。
同理，区间 [2,5] 和 [4,8] 也有交集，所以它们也必须在同一个组中。
所以总共有 4 种分组方案：
- 所有区间都在第 1 组。
- 所有区间都在第 2 组。
- 区间 [1,3] ，[2,5] 和 [4,8] 在第 1 个组中，[10,20] 在第 2 个组中。
- 区间 [1,3] ，[2,5] 和 [4,8] 在第 2 个组中，[10,20] 在第 1 个组中。
</pre>

#### 提示:
* <code>1 <= ranges.length <= 10<sup>5</sup></code>
* `ranges[i].length == 2`
* <code>0 <= start<sub>i</sub> <= end<sub>i</sub> <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
        let mut max_end = -1;
        let mut ret = 1;

        ranges.sort_unstable();

        for i in 0..ranges.len() {
            if ranges[i][0] > max_end {
                ret = (ret * 2) % 1_000_000_007;
            }
            max_end = max_end.max(ranges[i][1]);
        }

        ret
    }
}
```
