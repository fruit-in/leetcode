# 436. 寻找右区间
给你一个区间数组 `intervals` ，其中 <code>intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> ，且每个 <code>start<sub>i</sub></code> 都 **不同** 。

区间 `i` 的 **右侧区间** 可以记作区间 `j` ，并满足 <code>start<sub>j</sub> >= end<sub>i</sub></code> ，且 <code>start<sub>j</sub></code> **最小化** 。

返回一个由每个区间 `i` 的 **右侧区间** 的最小起始位置组成的数组。如果某个区间 `i` 不存在对应的 **右侧区间** ，则下标 `i` 处的值设为 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> intervals = [[1,2]]
<strong>输出:</strong> [-1]
<strong>解释:</strong> 集合中只有一个区间，所以输出-1。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> intervals = [[3,4],[2,3],[1,2]]
<strong>输出:</strong> [-1,0,1]
<strong>解释:</strong> 对于 [3,4] ，没有满足条件的“右侧”区间。
对于 [2,3] ，区间[3,4]具有最小的“右”起点;
对于 [1,2] ，区间[2,3]具有最小的“右”起点。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> intervals = [[1,4],[2,3],[3,4]]
<strong>输出:</strong> [-1,2,-1]
<strong>解释:</strong> 对于区间 [1,4] 和 [3,4] ，没有满足条件的“右侧”区间。
对于 [2,3] ，区间 [3,4] 有最小的“右”起点。
</pre>

#### 提示:
* <code>1 <= intervals.length <= 2 * 10<sup>4</sup></code>
* `intervals[i].length == 2`
* <code>-10<sup>6</sup> <= start<sub>i</sub> <= end<sub>i</sub> <= 10<sup>6</sup></code>
* 每个间隔的起点都 **不相同**

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[][]} intervals
# @return {Integer[]}
def find_right_interval(intervals)
  starts = []
  ends = []
  i = 0
  ret = [-1] * intervals.size

  (0...intervals.size).each do |j|
    starts.push([intervals[j][0], j])
    ends.push([intervals[j][1], j])
  end
  starts.sort!
  ends.sort!

  (0...ends.size).each do |j|
    i += 1 while i < ends.size && ends[j][0] > starts[i][0]
    break if i >= ends.size

    ret[ends[j][1]] = starts[i][1]
  end

  ret
end
```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut starts = vec![];
        let mut ends = vec![];
        let mut i = 0;
        let mut ret = vec![-1; intervals.len()];

        for j in 0..intervals.len() {
            starts.push((intervals[j][0], j));
            ends.push((intervals[j][1], j));
        }
        starts.sort_unstable();
        ends.sort_unstable();

        for j in 0..ends.len() {
            while i < ends.len() && ends[j].0 > starts[i].0 {
                i += 1;
            }
            if i >= ends.len() {
                break;
            }
            ret[ends[j].1] = starts[i].1 as i32;
        }

        ret
    }
}
```
