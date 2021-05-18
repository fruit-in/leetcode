# 436. Find Right Interval
You are given an array of `intervals`, where <code>intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> and each <code>start<sub>i</sub></code> is **unique**.

The **right interval** for an interval `i` is an interval `j` such that <code>start<sub>j</sub> >= end<sub>i</sub></code> and <code>start<sub>j</sub></code> is **minimized**.

Return *an array of **right interval** indices for each interval `i`*. If no **right interval** exists for interval `i`, then put `-1` at index `i`.

#### Example 1:
<pre>
<strong>Input:</strong> intervals = [[1,2]]
<strong>Output:</strong> [-1]
<strong>Explanation:</strong> There is only one interval in the collection, so it outputs -1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> intervals = [[3,4],[2,3],[1,2]]
<strong>Output:</strong> [-1,0,1]
<strong>Explanation:</strong> There is no right interval for [3,4].
The right interval for [2,3] is [3,4] since start<sub>0</sub> = 3 is the smallest start that is >= end<sub>1</sub> = 3.
The right interval for [1,2] is [2,3] since start<sub>1</sub> = 2 is the smallest start that is >= end<sub>2</sub> = 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> intervals = [[1,4],[2,3],[3,4]]
<strong>Output:</strong> [-1,2,-1]
<strong>Explanation:</strong> There is no right interval for [1,4] and [3,4].
The right interval for [2,3] is [3,4] since start<sub>2</sub> = 3 is the smallest start that is >= end<sub>1</sub> = 3.
</pre>

#### Constraints:
* <code>1 <= intervals.length <= 2 * 10<sup>4</sup></code>
* `intervals[i].length == 2`
* <code>-10<sup>6</sup> <= start<sub>i</sub> <= end<sub>i</sub> <= 10<sup>6</sup></code>
* The start point of each interval is **unique**.

## Solutions (Ruby)

### 1. Sort
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

## Solutions (Rust)

### 1. Sort
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
