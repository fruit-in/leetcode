# 57. 插入区间
给你一个 **无重叠的** ，按照区间起始端点排序的区间列表。

在列表中插入一个新的区间，你需要确保列表中的区间仍然有序且不重叠（如果有必要的话，可以合并区间）。

#### 示例 1:
<pre>
<strong>输入:</strong> intervals = [[1,3],[6,9]], newInterval = [2,5]
<strong>输出:</strong> [[1,5],[6,9]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
<strong>输出:</strong> [[1,2],[3,10],[12,16]]
<strong>解释:</strong> 这是因为新的区间 [4,8] 与 [3,5],[6,7],[8,10] 重叠。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> intervals = [], newInterval = [5,7]
<strong>输出:</strong> [[5,7]]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> intervals = [[1,5]], newInterval = [2,3]
<strong>输出:</strong> [[1,5]]
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> intervals = [[1,5]], newInterval = [2,7]
<strong>输出:</strong> [[1,7]]
</pre>

#### 提示:
* <code>0 <= intervals.length <= 10<sup>4</sup></code>
* `intervals[i].length == 2`
* <code>0 <= intervals[i][0] <= intervals[i][1] <= 10<sup>5</sup></code>
* `intervals` 根据 `intervals[i][0]` 按 **升序** 排列
* `newInterval.length == 2`
* <code>0 <= newInterval[0] <= newInterval[1] <= 10<sup>5</sup></code>

## 题解 (Ruby)

### 1. 题解
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

## 题解 (Rust)

### 1. 题解
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
