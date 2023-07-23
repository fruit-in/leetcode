# 57. Insert Interval
Given a set of *non-overlapping* intervals, insert a new interval into the intervals (merge if necessary).

You may assume that the intervals were initially sorted according to their start times.

#### Example 1:
<pre>
<strong>Input:</strong> intervals = [[1,3],[6,9]], newInterval = [2,5]
<strong>Output:</strong> [[1,5],[6,9]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
<strong>Output:</strong> [[1,2],[3,10],[12,16]]
<strong>Explanation:</strong> Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> intervals = [], newInterval = [5,7]
<strong>Output:</strong> [[5,7]]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> intervals = [[1,5]], newInterval = [2,3]
<strong>Output:</strong> [[1,5]]
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> intervals = [[1,5]], newInterval = [2,7]
<strong>Output:</strong> [[1,7]]
</pre>

#### Constraints:
* <code>0 <= intervals.length <= 10<sup>4</sup></code>
* `intervals[i].length == 2`
* <code>0 <= intervals[i][0] <= intervals[i][1] <= 10<sup>5</sup></code>
* `intervals` is sorted by `intervals[i][0]` in **ascending** order.
* `newInterval.length == 2`
* <code>0 <= newInterval[0] <= newInterval[1] <= 10<sup>5</sup></code>

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[][]} intervals
# @param {Integer[]} new_interval
# @return {Integer[][]}
def insert(intervals, new_interval)
  flag = true
  ret = []

  intervals.each do |interval|
    if interval[0] > new_interval[1]
      ret.push(new_interval) if flag
      ret.push(interval)
      flag = false
    elsif interval[1] < new_interval[0]
      ret.push(interval)
    else
      new_interval[0] = [new_interval[0], interval[0]].min
      new_interval[1] = [new_interval[1], interval[1]].max
    end
  end
  ret.push(new_interval) if flag

  ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut flag = true;
        let mut ret = vec![];

        for interval in intervals {
            if interval[0] > new_interval[1] {
                if flag {
                    ret.push(new_interval.clone());
                    flag = false;
                }
                ret.push(interval);
            } else if interval[1] < new_interval[0] {
                ret.push(interval);
            } else {
                new_interval[0] = new_interval[0].min(interval[0]);
                new_interval[1] = new_interval[1].max(interval[1]);
            }
        }
        if flag {
            ret.push(new_interval);
        }

        ret
    }
}
```
